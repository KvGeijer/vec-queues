/// A simple unbounded ringbuffer double ended queue, that re-allocates the buffer when full
pub(super) struct ReallocatingQueue<T: Sized + Clone> {
    /// A circular ring buffer, which can grow if full
    buffer: Vec<T>,

    /// The number of items in the queue
    size: usize,

    /// The index used for the next dequeue_first
    head: usize,

    /// The index for the next enqueue_last
    tail: usize,
}

impl<T: Sized + Clone> ReallocatingQueue<T> {
    /// Allocates a new, empty queue
    pub fn new() -> Self {
        Self::with_capacity(64)
    }

    /// Allocates a new, empty queue with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    /// Enqueue a new item at the front of the queue
    pub fn enqueue_first(&mut self, item: T) {
        // The vector is full, so we have to re-allocate it to a larger one
        if self.size == self.buffer.capacity() {
            self.reallocate(self.size * 2);
        }

        if self.head == 0 && self.buffer.len() < self.buffer.capacity() {
            // We still have not filled up the vector, which we need to do to put the item at the end
            while self.buffer.capacity() > self.buffer.len() {
                // TODO: By using unsafe, we could initialize the whole vector in the start, avoiding the distinction between push and insert as well
                self.buffer.push(item.clone())
            }

            self.head = self.buffer.len() - 1;
            self.size += 1;
        } else {
            // We don't have to worry about pushing, as the location is already filled
            self.head = (self.head - 1) % self.buffer.capacity();
            self.buffer[self.head] = item;
            self.size += 1;
        }
    }

    /// Enqueue a new item at the back of the queue
    pub fn enqueue_last(&mut self, item: T) {
        // The vector is full, so we have to re-allocate it to a larger one
        if self.tail == self.head && self.size > 0 {
            self.reallocate(self.size * 2);
        }

        if self.buffer.len() < self.buffer.capacity() {
            // We still have not filled up the vector, so use push
            self.buffer.push(item);
            self.tail = (self.tail + 1) % self.buffer.capacity();
            self.size += 1;
        } else {
            // We have filled up the vector, so now we wrap
            self.buffer[self.tail] = item;
            self.tail = (self.tail + 1) % self.buffer.capacity();
            self.size += 1;
        }
    }

    /// Reallocates the underlying vector to a different size, must be larger than the queue size
    fn reallocate(&mut self, capacity: usize) {
        if self.size > capacity {
            panic!(
                "(ReallocatingQueue) Internal error: Tried to re-allocate to a too small queue!"
            );
        }

        // Replace the old vector with a new one
        let mut old_buffer = std::mem::replace(&mut self.buffer, Vec::with_capacity(capacity));
        let old_capacity = old_buffer.capacity();

        // Add the items in their correct order from the old buffer to the new
        if self.head + self.size > old_capacity {
            // The items wrap around the end of the buffer
            self.buffer
                .extend(old_buffer.drain(self.head..old_capacity));
            self.buffer.extend(old_buffer.drain(0..self.tail));
        } else {
            // The items are contiguous without any wrapping
            self.buffer.extend(old_buffer.drain(self.head..self.tail));
        }

        // Make sure to update the head and tail
        self.head = 0;
        self.tail = self.size;
    }

    /// Dequeue the first item in the queue
    pub fn dequeue_first(&mut self) -> Option<T> {
        if self.size > 0 {
            let item = self.buffer[self.head].clone();
            self.head = (self.head + 1) % self.buffer.capacity();
            self.size -= 1;
            Some(item)
        } else {
            None
        }
    }

    /// Dequeue the last item in the queue
    pub fn dequeue_last(&mut self) -> Option<T> {
        if self.size > 0 {
            self.tail = (self.tail - 1) % self.buffer.capacity();
            let item = self.buffer[self.head].clone();
            self.size -= 1;
            Some(item)
        } else {
            None
        }
    }

    /// Peek at the last item in the queue
    pub fn peek_first(&self) -> Option<&T> {
        if self.size > 0 {
            Some(&self.buffer[self.head])
        } else {
            None
        }
    }

    /// Peek at the last item in the queue
    pub fn peek_last(&self) -> Option<&T> {
        if self.size > 0 {
            let index = (self.tail - 1) % self.buffer.capacity();
            Some(&self.buffer[index])
        } else {
            None
        }
    }

    /// Returns the number of items currently in the queue
    pub fn len(&self) -> usize {
        self.size
    }
}
