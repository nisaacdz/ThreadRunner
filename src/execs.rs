///! This module contains FixedThreadPool and its helper types
///! Worker and Msg are set to private
///!
use parking_lot::Mutex;

/// Describes the tasks that can be passed through the channels in `FixedThreadPool`
type Job = Box<dyn Send + 'static + Fn()>;

/// sender is the `Sender` end of the channel used for passing tasks to the workers
///
/// workers possess threads and are responsible for running the tasks they receiver from the channels in their own threads
///
/// Each worker possesses a superficial clone of a single `Receiver` end that they borrow mutably through `parking_lot::Mutex` borrow

pub struct FixedThreadPool {
    sender: std::sync::mpsc::Sender<Msg>,
    workers: Vec<Worker>,
}

impl FixedThreadPool {
    /// Creates a new FixedThreadPool with the specified number of worker threads.
    ///
    /// The executor service will spawn `size` worker threads, each of which will
    /// process tasks submitted to the service using the `execute` method.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads to create.
    ///
    /// # Returns
    ///
    /// A new `FixedThreadPool` object.
    ///
    /// # Panics
    ///
    /// This function will panic if the value of size is equal to zero
    ///
    /// # Example
    ///
    /// ```
    /// use thread_runner::execs::FixedThreadPool;
    ///
    /// let executor = FixedThreadPool::new(4);
    /// ```
    ///

    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0, "Executor service size must be non-zero");
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = std::sync::Arc::new(Mutex::new(receiver));
        for _ in 0..size {
            workers.push(Worker::new(receiver.clone()));
        }
        Self { sender, workers }
    }

    /// Executes the given closure as a `task` in a worker thread.
    ///
    /// This is achieved by sending the task to a pool of `workers`, who compete to execute it in their threads.
    ///
    /// Tasks submitted through the channel are executed in the order they are received (FIFO - First In, First Out).
    /// This means if the tasks outnumber the workers, the later tasks are suspended until the earlier tasks are executed.
    ///
    /// # Example
    ///
    /// ```
    /// use thread_runner::execs::FixedThreadPool;
    ///
    /// let executor = FixedThreadPool::new(4);
    ///
    /// for val in 0..1000 {
    ///     executor.execute(move || println!("{}", val));
    /// }
    ///
    /// executor.join();
    /// ```
    ///
    /// # Note
    ///
    /// If you want to wait for the submitted tasks to finish executing, you should call `join` on the executor service.

    pub fn execute<F: Send + 'static + Fn()>(&self, f: F) {
        let msg = Msg::Task(Box::new(f));
        self.sender.send(msg).unwrap()
    }

    /// Blocks the current thread until the `FixedThreadPool` completes all its executions
    ///
    pub fn join(self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Msg::Terminate).unwrap();
        }

        for Worker { thread } in self.workers {
            thread.join().unwrap();
        }
    }

    pub fn terminate(&self) {
        for worker in self.workers.iter() {
            worker.thread.thread().unpark();
        }
    }
}

/// A worker thread.
///
/// A `Worker` runs a loop that listens for tasks on a channel, and executes
/// each task as it arrives. It terminates when it receives a `Msg::Terminate`
/// message.

struct Worker {
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    /// New workers loop continuously in their own threads until they receive a Terminate message from the channel
    ///
    /// This terminate message is useful for joining the individual `JoinHandle<()>` objects during `join` of `FixedThreadPool`
    ///
    /// Calling unwrap on `recv()` is safe in this case because the channel will never hang up
    fn new(receiver: std::sync::Arc<Mutex<std::sync::mpsc::Receiver<Msg>>>) -> Self {
        Self {
            thread: std::thread::spawn(move || loop {
                let msg = receiver.lock().recv().unwrap();
                match msg {
                    Msg::Terminate => break,
                    Msg::Task(job) => job(),
                }
            }),
        }
    }
}

/// Represents a message that can be sent through the executor's channel.
enum Msg {
    /// Instructs the worker to terminate its execution.
    Terminate,
    /// Represents a task to be executed by the worker.
    Task(Job),
}
