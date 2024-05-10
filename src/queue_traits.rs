/// A trait to simplify generalizing over unbounded fifo queues
pub trait FifoQueue<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A trait to simplify generalizing over unbounded double-ended queues
pub trait Deque<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn enqueue_first(&mut self, item: T);
    fn enqueue_last(&mut self, item: T);
    fn dequeue_first(&mut self) -> Option<T>;
    fn dequeue_last(&mut self) -> Option<T>;
    fn peek_first(&self) -> Option<&T>;
    fn peek_last(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
