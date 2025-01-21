//! Contains the [`AlgorithmExecutor`] struct, which is a stream that will yield
//! the midway results of the algorithm as they come in, the last result being
//! the final algorithm result.

use std::sync::Arc;

use futures_core::Stream;
use futures_util::{
    lock::Mutex,
    FutureExt,
};
use gloo_timers::future::TimeoutFuture;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    recalculate_map::{
        recalculate_map,
        Updater,
    },
    AlgorithmSettings,
};
use crate::{
    models::Map,
    utils::{
        IDData,
        IDManager,
    },
    Error,
};

/// The response from the algorithm.
#[derive(Clone, Serialize, Deserialize)]
pub struct AlgorithmResponse {
    /// If the algorithm ran successfully.
    pub success: bool,
    /// The Map outputted by the algorithm.
    pub map: Map,
    /// The data for the [`IDManager`] after the algorithm has run, ensuring the
    /// main thread will not create IDs in conflict with those in the map.
    pub id_manager_data: IDData,
    /// If an error occurred during the algorithm, this contains it.
    pub error: Option<Error>,
}

/// The inner state of the executor.
#[derive(Clone)]
struct ExecutorState {
    /// The most recent result of the algorithm.
    last_res: Option<AlgorithmResponse>,
    /// If the algorithm is done.
    done: bool,
    /// The waker for the stream.
    waker: Option<std::task::Waker>,
    /// Any error that occurred during the algorithm.
    error: Option<Error>,
}

/// The executor for the algorithm.
/// This is a stream that will yield the midway results of the algorithm as they
/// come in, the last result being the final algorithm result.
#[derive(Clone)]
pub struct AlgorithmExecutor {
    /// The inner state of the executor, wrapped in an arc and mutex to allow
    /// updating from one async thread and read from another.
    inner: Arc<Mutex<ExecutorState>>,
    /// If midway results should be returned.
    midway_updates: bool,
}

impl AlgorithmExecutor {
    /// Create a new [`AlgorithmExecutor`] with the given settings, map and if
    /// midway results should be returned.
    pub fn new(settings: AlgorithmSettings, map: Map, midway_updates: bool) -> Self {
        let executor = Self {
            inner: Arc::new(Mutex::new(ExecutorState {
                last_res: None,
                done: false,
                waker: None,
                error: None,
            })),
            midway_updates,
        };

        // Spawn the algorithm on a separate async thread.
        let closure_executor = executor.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let recalc_executor = closure_executor.clone();
            let mut map = map.clone();

            let res = recalculate_map(
                settings,
                &mut map,
                // If midway updates are enabled, send updates to the stream.
                if midway_updates {
                    Updater::Updater(Arc::new(Box::new(
                        move |map: Map, id_manager_data: IDData| {
                            let recalc_executor = recalc_executor.clone();
                            async move {
                                recalc_executor
                                    .update_last_res(map, id_manager_data, true)
                                    .await;
                                recalc_executor
                                    .wake()
                                    .await;
                            }
                            .boxed_local()
                        },
                    )))
                } else {
                    Updater::NoUpdates
                },
            )
            .await;

            closure_executor
                .update_last_res(map, IDManager::to_data(), res.is_ok())
                .await;
            closure_executor
                .set_error(res.err())
                .await;
            closure_executor
                .mark_done()
                .await;
            closure_executor
                .wake()
                .await;
        });

        executor
    }

    /// Update the last result of the algorithm.
    async fn update_last_res(&self, map: Map, id_manager_data: IDData, success: bool) {
        let res = AlgorithmResponse {
            success,
            map,
            id_manager_data,
            error: None,
        };

        self.inner
            .lock()
            .await
            .last_res
            .replace(res);
    }

    /// Pop the last result of the algorithm.
    async fn pop_last_res(&mut self) -> Option<AlgorithmResponse> {
        self.inner
            .lock()
            .await
            .last_res
            .take()
    }

    /// Set the error of the algorithm.
    async fn set_error(&self, error: Option<Error>) {
        self.inner
            .lock()
            .await
            .error = error;
    }

    /// Get the error of the algorithm.
    async fn get_error(&self) -> Option<Error> {
        self.inner
            .lock()
            .await
            .error
            .clone()
    }

    /// Mark the algorithm and thus stream as done.
    async fn mark_done(&self) {
        self.inner
            .lock()
            .await
            .done = true;
    }

    /// Get if the algorithm is done.
    async fn get_done(&self) -> bool {
        self.inner
            .lock()
            .await
            .done
    }

    /// Set the stream waker.
    async fn set_waker(&mut self, waker: std::task::Waker) {
        self.inner
            .lock()
            .await
            .waker
            .replace(waker);
    }

    /// Wake the stream.
    async fn wake(&self) {
        if let Some(waker) = &self
            .inner
            .lock()
            .await
            .waker
        {
            waker.wake_by_ref();
        }

        if self.midway_updates {
            TimeoutFuture::new(1).await;
        }
    }
}

impl Stream for AlgorithmExecutor {
    type Item = AlgorithmResponse;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // Set the async waker.
        this.set_waker(
            cx.waker()
                .clone(),
        )
        .now_or_never();

        // Get the most recent result and check if the algorithm is done.
        let res = this
            .pop_last_res()
            .now_or_never();
        let error = this
            .get_error()
            .now_or_never()
            .flatten();
        let done = this
            .get_done()
            .now_or_never()
            .unwrap_or(false);

        match res {
            Some(Some(mut res)) => {
                res.error = error;
                std::task::Poll::Ready(Some(res))
            },
            Some(None) if done => std::task::Poll::Ready(None),
            _ => std::task::Poll::Pending,
        }
    }
}
