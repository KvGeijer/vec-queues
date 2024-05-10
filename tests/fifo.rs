use itertools::repeat_n;
use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Uniform,
    },
    Rng,
};
use vec_queues::{FifoQueue, ReallocatingFifo};

/// Function that checks if the queue is equal to an iterator of items
fn dequeue_iter_test<T: Sized + Clone + Eq + std::fmt::Debug>(
    mut queue: impl FifoQueue<T>,
    items: impl IntoIterator<Item = T>,
) {
    for item in items {
        assert_eq!(Some(item), queue.dequeue());
    }
    assert!(queue.is_empty());
}

#[test]
/// First enqueues a bunch of items in fifo order, then tests if they are dequeued in the same one
fn sync_enq_deq() {
    let iters: [Vec<i32>; 4] = [
        (0..1024).collect(),
        (-1024..10000).collect(),
        rand::thread_rng()
            .sample_iter(&Uniform::from(-10..10))
            .take(10000)
            .collect(),
        rand::thread_rng()
            .sample_iter(&Uniform::from(-100000..100000))
            .take(10000)
            .collect(),
    ];

    for iter in iters {
        let mut queue = ReallocatingFifo::with_capacity(10);
        for item in iter.clone() {
            queue.enqueue(item);
        }
        dequeue_iter_test(queue, iter)
    }
}

/// At random enqueues an item from the iterator, or dequeue an item and compares against ground truth
fn random_op_iter_test<T: Sized + Clone + Eq + std::fmt::Debug + SampleUniform, Q: FifoQueue<T>>(
    enq_ops: impl IntoIterator<Item = bool>,
    item_range: impl SampleRange<T> + Clone,
) {
    let mut queue = Q::new();
    let mut vec = Vec::with_capacity(10000);
    let mut vec_head = 0;

    for op in enq_ops {
        if op {
            // Enqueue an item
            let item = item_range.clone().sample_single(&mut rand::thread_rng());
            vec.push(item.clone());
            queue.enqueue(item);
        } else {
            // Dequeue an item
            if queue.is_empty() {
                assert_eq!(vec.len(), vec_head);
                assert_eq!(queue.dequeue(), None);
            } else {
                assert_eq!(queue.dequeue().as_ref(), vec.get(vec_head));
                vec_head += 1;
            }
        }
    }

    // At the end, just dequeue all items and see that it checks out
    dequeue_iter_test(queue, vec.drain(vec_head..));
}

fn gen_bools(n: usize, p: f64) -> Vec<bool> {
    (0..n).map(|_| rand::thread_rng().gen_bool(p)).collect()
}

#[test]
/// At random enqueue or dequeue items from the queue
fn random_enq_deq() {
    let enq_op_iters: Vec<Vec<bool>> = vec![
        repeat_n(false, 200).collect(),
        repeat_n(true, 100)
            .chain(gen_bools(1000, 0.5).into_iter())
            .collect(),
        repeat_n(true, 100)
            .chain(gen_bools(1000, 0.5).into_iter())
            .collect(),
        gen_bools(10000, 0.52)
            .into_iter()
            .chain(gen_bools(1000, 0.4).into_iter())
            .collect(),
    ];

    for op_iter in enq_op_iters {
        random_op_iter_test::<i32, ReallocatingFifo<i32>>(op_iter, -256..256);
    }
}
