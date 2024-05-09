use crate::FifoQueue;

use super::internal_queue::ReallocatingQueue;

/// A Fifo queue based on a ring-buffer, that re-allocates when full
pub struct ReallocatingFifo<T: Clone + Sized> {
    /// Delegate functions to more generic queue implementation
    queue: ReallocatingQueue<T>,
}

impl<T: Sized + Clone> ReallocatingFifo<T> {
    /// Enqueue an item into the queue
    pub fn enqueue(&mut self, item: T) {
        // More efficient to enqueue last and dequeue first in simple implementation
        self.queue.enqueue_last(item)
    }

    /// Try to dequeue the oldest item in the queue
    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.dequeue_first()
    }

    /// The number of items currently in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Tries to peek at the oldest item in the queue
    pub fn peek(&self) -> Option<&T> {
        self.queue.peek_last()
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

impl<T: Clone + Sized> FifoQueue<T> for ReallocatingFifo<T> {
    fn enqueue(&mut self, item: T) {
        self.enqueue(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.dequeue()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn peek(&self) -> Option<&T> {
        self.peek()
    }

    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}
