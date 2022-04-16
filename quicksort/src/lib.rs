pub mod sort {
    /// Computes the media for a given vector of numbers.
    /// The media is the value separating the higher half from
    /// the lower half of a data sample.
    ///
    /// # Examples
    ///
    /// A data set with an odd number of values:
    /// ```
    /// use quicksort::sort::median;
    /// let vec = vec![1.0,3.0,3.0,6.0,7.0,8.0,9.0];
    /// let median = median(&vec[..]);
    /// assert_eq!(median, 6.0);
    /// ```
    ///
    /// A data set with an even number of values:
    /// ```
    /// use quicksort::sort::median;
    /// let vec = vec![1.0,2.0,3.0,4.0,5.0,6.0,8.0,9.0];
    /// let median = median(&vec[..]);
    /// assert_eq!(median, 4.5);
    /// ```
    pub fn median(vec: &[f64]) -> f64 {
        let len = vec.len();
        return if len % 2 == 0 {
            (vec[len / 2 - 1] + vec[len / 2]) / 2f64
        } else {
            vec[(len + 1) / 2 - 1]
        };
    }

    fn swap(vec: &mut [f64], a: usize, b: usize) {
        let old_a = vec[a];
        vec[a] = vec[b];
        vec[b] = old_a;
    }

    fn partition_hoare(vec: &mut [f64], mut left: usize, mut right: usize) -> usize {
        let pivot = vec[vec.len() / 2];

        loop {
            while vec[left] < pivot {
                left += 1;
            }
            while vec[right] > pivot {
                right -= 1;
            }
            if left >= right {
                break;
            }
            swap(vec, left, right);
        }

        return left;
    }

    #[cfg(test)]
    mod tests {
        use crate::sort;

        fn partition_hoare(vec: &mut [f64]) -> usize {
            let left = 0;
            let right = vec.len() - 1;
            sort::partition_hoare(vec, left, right)
        }

        #[test]
        fn hoare_even_sorted() {
            let mut vec = vec![1.0, 2.4, 3.0, 7.0];
            let pivot = partition_hoare(&mut vec);
            assert_eq!(pivot, 2);
            assert_eq!(vec, [1.0, 2.4, 3.0, 7.0]);
        }

        #[test]
        fn hoare_odd_sorted() {
            let mut vec = vec![2.3, 3.0, 4.0];
            let pivot = partition_hoare(&mut vec);
            assert_eq!(pivot, 1);
            assert_eq!(vec, [2.3, 3.0, 4.0]);
        }

        #[test]
        fn hoare_even_unsorted() {
            let mut vec = vec![1.0, 7.1, 2.2, 8.0];
            let pivot = partition_hoare(&mut vec);
            assert_eq!(pivot, 1);
            assert_eq!(vec, [1.0, 2.2, 7.1, 8.0]);
        }

        #[test]
        fn hoare_odd_unsorted() {
            let mut vec = vec![9.2, 3.1, 4.0];
            let pivot = partition_hoare(&mut vec);
            assert_eq!(pivot, 0);
            assert_eq!(vec, [3.1, 9.2, 4.0]);
        }
    }
}

pub mod threads {

    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::thread::{sleep, JoinHandle};
    use std::time::Duration;

    pub enum ThreadState {
        IDLE,
        WORKING,
    }

    pub struct PoolState {
        thread_states: Mutex<Vec<ThreadState>>,
    }

    pub struct Pool<T> {
        txs: Vec<Sender<T>>,
        pub threads: Vec<JoinHandle<()>>,
        state: Arc<PoolState>,
    }

    type ThreadClosure = dyn Fn() + Send + 'static;

    impl Pool<Box<ThreadClosure>> {
        pub fn new(size: usize) -> Self {
            let mut txs = vec![];
            let mut threads = vec![];
            let mut thread_states = vec![];
            for _ in 0..size {
                thread_states.push(ThreadState::IDLE);
            }

            let state = Arc::new(PoolState {
                thread_states: Mutex::from(thread_states),
            });

            for id in 0..size {
                let (tx, rx) = mpsc::channel();

                txs.push(tx);

                let state = Arc::clone(&state);
                threads.push(thread::spawn(move || {
                    Pool::worker_thread(state, id, rx);
                }));
            }

            Pool {
                txs,
                threads,
                state,
            }
        }

        pub fn execute(&self, block: Box<ThreadClosure>) {
            let thread_id;

            {
                let thread_states = self.state.thread_states.lock().unwrap();
                thread_id = thread_states
                    .iter()
                    .position(|thread_state| match thread_state {
                        ThreadState::IDLE => true,
                        ThreadState::WORKING => false,
                    })
                    .or_else(|| Option::Some(0))
                    .unwrap();
            }

            println!("Submitting task to thread {}...", thread_id);

            // TODO: Cover case in which sending fails
            self.txs[thread_id].send(block);

            // We sleep to give the opportunity the thread to acquire the lock of the thread state. If we don't
            // wait, we will end up having the lock acquired by all the calls to "execute".
            sleep(Duration::new(0, 100))
        }

        fn worker_thread(state: Arc<PoolState>, id: usize, rx: Receiver<Box<ThreadClosure>>) {
            loop {
                for closure in rx.iter() {
                    {
                        let mut state = state.thread_states.lock().unwrap();

                        (*state)[id] = ThreadState::WORKING;
                        println!("Thread {} working...", id);
                    }

                    closure();

                    {
                        let mut state = state.thread_states.lock().unwrap();
                        (*state)[id] = ThreadState::IDLE;
                        println!("Thread {} idle...", id);
                    }
                }
            }
        }
    }
}
