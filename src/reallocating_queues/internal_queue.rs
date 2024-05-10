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
        if self.size == self.capacity() {
            self.reallocate(self.size * 2);
        }

        if self.head == 0 && self.buffer.len() < self.capacity() {
            // We still have not filled up the vector, which we need to do to put the item at the end
            while self.capacity() > self.buffer.len() {
                // TODO: By using unsafe, we could initialize the whole vector in the start, avoiding the distinction between push and insert as well
                self.buffer.push(item.clone())
            }
        } else {
            // We don't have to worry about pushing, as the location is already filled
            self.buffer[self.head] = item;
        }
        self.head = self.dec_head();
        self.size += 1;
    }

    /// Enqueue a new item at the back of the queue
    pub fn enqueue_last(&mut self, item: T) {
        // The vector is full, so we have to re-allocate it to a larger one
        if self.size == self.capacity() {
            self.reallocate(self.size * 2);
        }

        if self.buffer.len() < self.capacity() {
            // We still have not filled up the vector, so use push
            self.buffer.push(item);
        } else {
            // We have filled up the vector, so now we wrap
            self.buffer[self.tail] = item;
        }
        self.tail = self.inc_tail();
        self.size += 1;
    }

    /// Reallocates the underlying vector to a different size, must be larger than the queue size
    fn reallocate(&mut self, capacity: usize) {
        if self.size > capacity {
            panic!(
                "(ReallocatingQueue) Internal error: Tried to re-allocate to a too small queue!"
            );
        }

        // Replace the old vector with a new one
        let mut new_buffer = Vec::with_capacity(capacity);

        // Add the items in their correct order from the old buffer to the new
        if self.head + self.size > self.capacity() {
            // The items wrap around the end of the buffer
            new_buffer.extend(self.buffer.drain(self.head..self.capacity()));
            new_buffer.extend(self.buffer.drain(0..self.tail));
        } else {
            // The items are contiguous without any wrapping
            new_buffer.extend(self.buffer.drain(self.head..=self.dec_tail()));
        }

        // Make sure to update the head and tail
        self.buffer = new_buffer;
        self.head = 0;
        self.tail = self.size;
    }

    /// Dequeue the first item in the queue
    pub fn dequeue_first(&mut self) -> Option<T> {
        if self.size > 0 {
            let item = self.buffer[self.head].clone();
            self.head = self.inc_head();
            self.size -= 1;
            Some(item)
        } else {
            None
        }
    }

    /// Dequeue the last item in the queue
    pub fn dequeue_last(&mut self) -> Option<T> {
        if self.size > 0 {
            self.tail = self.dec_tail();
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
            let index = self.dec_tail();
            Some(&self.buffer[index])
        } else {
            None
        }
    }

    /// Returns the number of items currently in the queue
    pub fn len(&self) -> usize {
        self.size
    }

    /// Gets the capacity of the current buffer
    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Returns tail incremented by 1, modulo self.capacity()
    fn inc_tail(&self) -> usize {
        wrapping_inc(self.tail, self.capacity())
    }

    /// Returns tail decremented by 1, modulo self.capacity()
    fn dec_tail(&self) -> usize {
        wrapping_dec(self.tail, self.capacity())
    }

    /// Returns head incremented by 1, modulo self.capacity()
    fn inc_head(&self) -> usize {
        wrapping_inc(self.head, self.capacity())
    }

    /// Returns head decremented by 1, modulo self.capacity()
    fn dec_head(&self) -> usize {
        wrapping_dec(self.head, self.capacity())
    }
}

/// Subtracts base by 1 in modular arithmetic with wrap.
fn wrapping_dec(mut base: usize, wrap: usize) -> usize {
    if base >= wrap {
        base = base % wrap;
    }

    if base != 0 {
        base - 1
    } else {
        wrap - 1
    }
}

/// Increments base by 1 in modular arithmetic with wrap.
fn wrapping_inc(mut base: usize, wrap: usize) -> usize {
    if base >= wrap {
        base = base % wrap;
    }

    if base + 1 != wrap {
        base + 1
    } else {
        0
    }
}
