use std::{
    collections::VecDeque,
    sync::{
        LazyLock,
        Mutex,
    },
};

use crate::models::Map;

static PAST_STACK: LazyLock<Mutex<BoundedStack<5, Map>>> =
    LazyLock::new(|| Mutex::new(BoundedStack::new()));
static FUTURE_STACK: LazyLock<Mutex<BoundedStack<5, Map>>> =
    LazyLock::new(|| Mutex::new(BoundedStack::new()));

struct BoundedStack<const N: usize, T> {
    stack: VecDeque<T>,
}

impl<const N: usize, T> BoundedStack<N, T> {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.stack
            .push_back(item);
        if self
            .stack
            .len()
            > N
        {
            self.stack
                .pop_front();
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack
            .pop_back()
    }

    fn clear(&mut self) {
        self.stack
            .clear();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HistoryState {}

impl HistoryState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn undo(&self, current: Map) -> Option<Map> {
        let map = PAST_STACK
            .lock()
            .unwrap()
            .pop()?;
        FUTURE_STACK
            .lock()
            .unwrap()
            .push(current);
        Some(map)
    }

    pub fn redo(&self, current: Map) -> Option<Map> {
        let map = FUTURE_STACK
            .lock()
            .unwrap()
            .pop()?;
        PAST_STACK
            .lock()
            .unwrap()
            .push(current);
        Some(map)
    }
}

pub(super) fn push_past_map(map: Map) {
    PAST_STACK
        .lock()
        .unwrap()
        .push(map);
    FUTURE_STACK
        .lock()
        .unwrap()
        .clear();
}
