use crate::queue_traits::Deque;

use super::internal_queue::ReallocatingQueue;

/// A double ended queue based on a ring-buffer, that re-allocates when full
pub struct ReallocatingDeque<T: Clone + Sized> {
    /// Delegate functions to more generic queue implementation
    queue: ReallocatingQueue<T>,
}

impl<T: Sized + Clone> ReallocatingDeque<T> {
    /// Enqueue an item at the end of the queue
    pub fn enqueue_last(&mut self, item: T) {
        self.queue.enqueue_last(item)
    }

    /// Enqueue an item at the front of the queue
    pub fn enqueue_first(&mut self, item: T) {
        self.queue.enqueue_first(item)
    }

    /// Try to dequeue the first item in the queue
    pub fn dequeue_first(&mut self) -> Option<T> {
        self.queue.dequeue_first()
    }

    /// Try to dequeue the last item in the queue
    pub fn dequeue_last(&mut self) -> Option<T> {
        self.queue.dequeue_last()
    }

    /// Tries to peek at the first item in the queue
    pub fn peek_first(&self) -> Option<&T> {
        self.queue.peek_first()
    }

    /// Tries to peek at the last item in the queue
    pub fn peek_last(&self) -> Option<&T> {
        self.queue.peek_last()
    }

    /// The number of items currently in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Created a new, empty queue
    pub fn new() -> Self {
        Self {
            queue: ReallocatingQueue::new(),
        }
    }

    /// Created a new, empty queue. Can at least enqueue _capacity_ items before re-allocating memory
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: ReallocatingQueue::with_capacity(capacity),
        }
    }
}

impl<T: Clone + Sized> Deque<T> for ReallocatingDeque<T> {
    fn enqueue_first(&mut self, item: T) {
        self.enqueue_first(item)
    }

    fn enqueue_last(&mut self, item: T) {
        self.enqueue_last(item)
    }

    fn dequeue_first(&mut self) -> Option<T> {
        self.dequeue_first()
    }

    fn dequeue_last(&mut self) -> Option<T> {
        self.dequeue_last()
    }

    fn peek_first(&self) -> Option<&T> {
        self.peek_first()
    }

    fn peek_last(&self) -> Option<&T> {
        self.peek_last()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}
