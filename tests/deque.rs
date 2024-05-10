use rand::{distributions::Uniform, Rng};
use vec_queues::{Deque, ReallocatingDeque};

/// Function that checks if the queue is equal to an iterator of items
fn dequeue_vec_test<T: Sized + Clone + Eq + std::fmt::Debug>(
    mut queue: ReallocatingDeque<T>,
    mut items: Vec<T>,
) {
    let mut vec_head = 0;

    while items.len() > vec_head {
        // Dequeue from one of the sides at random
        if rand::thread_rng().gen_bool(0.5) {
            // Deq at head
            assert_eq!(items.get(vec_head), queue.dequeue_first().as_ref());
            vec_head += 1;
        } else {
            // Deq at tail
            assert_eq!(items.pop().as_ref(), queue.dequeue_last().as_ref());
        }
    }
    assert!(queue.is_empty());
}

#[test]
/// First enqueues a bunch of items at both ends, then tests if they are dequeued correctly randomly from both ends
fn sync_enq_deq() {
    let iters: Vec<Vec<i32>> = vec![
        (0..100).collect(),
        (-1024..1000).collect(),
        rand::thread_rng()
            .sample_iter(&Uniform::from(-10..10))
            .take(1000)
            .collect(),
        rand::thread_rng()
            .sample_iter(&Uniform::from(-100000..100000))
            .take(1000)
            .collect(),
    ];

    for items in iters {
        let mut queue = ReallocatingDeque::with_capacity(10);
        let start_ind = rand::thread_rng().gen_range(0..items.len());
        queue.enqueue_first(items[start_ind]);
        let mut left = start_ind; // One above to prevent underflow
        let mut right = start_ind + 1;

        // Enqueue the items to get a corresponding state to the vec, but from both sides at random
        while left != 0 && right < items.len() {
            if rand::thread_rng().gen_bool(0.5) {
                // Add last
                queue.enqueue_last(items[right]);
                right += 1;
            } else {
                // Add first
                left -= 1;
                queue.enqueue_first(items[left]);
            }
        }

        for item in items[0..left].iter().rev() {
            queue.enqueue_first(*item);
        }

        for item in items[right..].iter() {
            queue.enqueue_last(*item);
        }

        // Test that they are actually the same (and deque works)
        dequeue_vec_test(queue, items)
    }
}
