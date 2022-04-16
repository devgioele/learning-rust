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
    /// let values = vec![1.0,3.0,3.0,6.0,7.0,8.0,9.0];
    /// let median = median(&values[..]);
    /// assert_eq!(median, 6.0);
    /// ```
    ///
    /// A data set with an even number of values:
    /// ```
    /// use quicksort::sort::median;
    /// let values = vec![1.0,2.0,3.0,4.0,5.0,6.0,8.0,9.0];
    /// let median = median(&values[..]);
    /// assert_eq!(median, 4.5);
    /// ```
    pub fn median(values: &[f64]) -> f64 {
        let len = values.len();
        return if len % 2 == 0 {
            (values[len / 2 - 1] + values[len / 2]) / 2f64
        } else {
            values[(len + 1) / 2 - 1]
        };
    }

    pub fn median_rec(values: &[f64], degree: u32) -> f64 {
        let len = values.len();

        // If there are not enough elements
        /*
        degree   | values.len()
        -----------------------
        1        | >= 1 = 3^0
        2        | >= 3 = 3^1
        3        | >= 9 = 3^2
         */
        if len < 3usize.pow(degree - 1) {
            panic!(
                "Not enough elements to compute the median of degree {}!",
                degree
            );
        }

        // If base case reached
        if degree == 1 {
            return median(values);
        }
        // Continue by induction
        return median(&[
            median_rec(&values[0..len / 3], degree - 1),
            median_rec(&values[len / 3..2 * len / 3], degree - 1),
            median_rec(&values[2 * len / 3..len], degree - 1),
        ]);
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
        threads: Vec<JoinHandle<()>>,
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

#[cfg(test)]
mod tests {
    use crate::sort::{median, median_rec};

    #[test]
    fn median_base_case_even() {
        let values = vec![1.0, 2.0, 3.7, 4.0, 5.0, 6.0, 8.0, 9.0];
        assert_eq!(median(&values), median_rec(&values, 1));
    }

    #[test]
    fn median_base_case_odd() {
        let values = vec![1.0, 3.0, 4.0, 5.1, 6.0, 8.0, 9.0];
        assert_eq!(median(&values), median_rec(&values, 1));
    }
}
