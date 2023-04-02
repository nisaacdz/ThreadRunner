

/// # AsyncRuntime
/// 
/// An asynchonous runtime that provides simple API for integrating async functions
/// into your synchronous code.
/// 
/// This struct utilizes `tokio`'s `rt` and `rt-multi-thread` features.
/// 
/// # Polling & Executing
/// * `poll` - for polling futures to completion
/// * `execute` - pushes the task into the runtime for scheduling and execution
///
/// # Examples
///
/// ```
/// use thread_runner::{AsyncRuntime, AsyncFlavor};
/// use std::time::Duration;
///
/// // Create a new runtime that executes all tasks in the current thread.
/// let runtime = AsyncRuntime::new(AsyncFlavor::CurrentThread);
///
/// // Spawn a future on the runtime.
/// runtime.execute(async {
///     // Do some asynchronous work here...
/// });
///
/// // Poll a future on the runtime and block until it completes.
/// let result = runtime.poll(async {
///     // Do some asynchronous work here and return a value...
///     42
/// });
///
/// // Shut down the runtime after a specified timeout duration.
/// runtime.terminate(Duration::from_secs(1));
/// ```
/// # Note
/// The Runtime automatically shutsdown after completing all the futures
/// but some futures may be indefinite. Such is when termiate becomes usefull
///
pub struct AsyncRuntime {
    runtime: tokio::runtime::Runtime,
}

impl AsyncRuntime {
    /// Creates a new `AsyncRuntime` instance based on the given `properties`.
    ///
    /// # Arguments
    ///
    /// * `properties` - A `AsyncFlavor` enum that specifies the type of runtime to create.
    ///
    /// # Returns
    ///
    /// A new `AsyncRuntime` instance with the specified configuration.
    pub fn new(properties: AsyncFlavor) -> Self {
        Self {
            runtime: match properties {
                AsyncFlavor::CurrentThread => tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
                AsyncFlavor::WorkerThreads(size) => {
                    tokio::runtime::Builder::new_multi_thread()
                        .worker_threads(size)
                        .enable_all()
                        .build()
                        .unwrap()
                }
                AsyncFlavor::AllThreads => tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
            },
        }
    }

    /// Schedules the given future `F` to be executed on the runtime.
    ///
    /// The `execute` method spawns a new task in the runtime and runs it asynchronously.
    ///
    /// This function is non-blocking.
    /// # Examples
    ///
    /// ```
    /// use thread_runner::{AsyncRuntime, AsyncFlavor};
    ///
    /// let runtime = AsyncRuntime::new(AsyncFlavor::CurrentThread);
    /// runtime.execute(async {
    ///     println!("This will execute on a single thread runtime.");
    /// });
    /// ```

    pub fn execute<F: Send + 'static + std::future::Future>(&self, f: F)
    where
        F::Output: Send + 'static,
    {
        self.runtime.spawn(f);
    }
    /// Polls the Future to completion.
    ///
    /// The `poll` method blocks the current thread and waits for the completion of the future.
    ///
    ///
    /// # Arguments
    /// - `f` the future to execute
    ///
    ///
    /// # Returns
    /// - `T` the Output of `f`
    ///
    ///  `f: F: Future<Output = T>`
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use thread_runner::{AsyncRuntime, AsyncFlavor};
    ///
    /// let runtime = AsyncRuntime::new(AsyncFlavor::WorkerThreads(3));
    /// let result = runtime.poll(async {
    ///     42
    /// });
    /// assert_eq!(result, 42);
    /// ```
    pub fn poll<T, F: std::future::Future<Output = T>>(&self, f: F) -> T {
        self.runtime.block_on(f)
    }

    /// Terminate the runtime and wait for all remaining tasks to complete.
    ///
    /// The `terminate` method initiates a graceful shutdown of the runtime, giving all
    /// active tasks a chance to complete before shutting down. If any task fails to complete
    /// within the specified timeout duration, the runtime will forcibly shut down.
    ///
    /// # Examples
    ///
    /// ```
    /// use thread_runner::{AsyncRuntime, AsyncFlavor};
    /// use std::time::Duration;
    ///
    /// let runtime = AsyncRuntime::new(AsyncFlavor::AllThreads);
    /// runtime.execute(async {
    ///     println!("This will execute on a single thread runtime.");
    /// });
    /// runtime.terminate(Duration::from_secs(1));
    /// ```
    ///
    /// # Note
    ///
    /// By default, the runtime will wait for all futures to complete before shutting down, which can be unnecessarily time-consuming in some situations. For these cases, it's best to use the `terminate` method to specify a timeout for the shutdown.

    pub fn terminate(self, timeout: std::time::Duration) {
        self.runtime.shutdown_timeout(timeout)
    }
}

/// Specifies the type of Tokio runtime to create.
pub enum AsyncFlavor {
    /// Creates a Tokio runtime with a single thread.
    CurrentThread,

    /// Creates a Tokio runtime with all available threads.
    AllThreads,

    /// Creates a Tokio runtime with the specified number of worker threads.
    WorkerThreads(usize),
}
