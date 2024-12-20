//! Contains everything for being able to redo and undo map changes.

use std::{
    collections::VecDeque,
    sync::{
        LazyLock,
        Mutex,
    },
};

use crate::models::Map;

/// The stack that contains the past maps.
static PAST_STACK: LazyLock<Mutex<BoundedStack<5, Map>>> =
    LazyLock::new(|| Mutex::new(BoundedStack::new()));
/// The stack that contains maps with changes that were undone by the user.
static FUTURE_STACK: LazyLock<Mutex<BoundedStack<5, Map>>> =
    LazyLock::new(|| Mutex::new(BoundedStack::new()));

/// A stack that is bounded to a certain size.
struct BoundedStack<const N: usize, T> {
    /// The stack itself.
    stack: VecDeque<T>,
}

impl<const N: usize, T> BoundedStack<N, T> {
    /// Create a new bounded stack.
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    /// Push an item onto the stack.
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

    /// Pop an item off the stack.
    fn pop(&mut self) -> Option<T> {
        self.stack
            .pop_back()
    }

    /// Clear the stack.
    fn clear(&mut self) {
        self.stack
            .clear();
    }
}

/// Contains everything for being able to redo and undo map changes.
#[derive(Debug, Copy, Clone)]
pub struct HistoryState {}

impl HistoryState {
    /// Returns the last map that was stored.
    pub fn undo(current: Map) -> Option<Map> {
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

    /// Returns the last map that was undone.
    pub fn redo(current: Map) -> Option<Map> {
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

/// Pushes the current map onto the past stack and clears the future stack.
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
