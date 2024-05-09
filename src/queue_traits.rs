/// A trait to simplify generalizing over unbounded fifo queues
pub trait FifoQueue<T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn peek(&self) -> Option<&T>;
}

/// A trait to simplify generalizing over unbounded double-ended queues
pub trait Deque<T> {
    fn enqueue_first(&mut self, item: T);
    fn enqueue_last(&mut self, item: T);
    fn dequeue_first(&mut self) -> Option<T>;
    fn dequeue_last(&mut self) -> Option<T>;
    fn peek_first(&self) -> Option<&T>;
    fn peek_last(&self) -> Option<&T>;
    fn len(&self) -> usize;
}
