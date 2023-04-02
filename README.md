# ThreadRunner

ThreadRunner is a Rust library for executing tasks concurrently.

It currently contains:

- **ThreadPool:** Use a pool of threads to handle expensive computations.


- **AsyncRuntime:** Seemlessly integrate async code into your synchronous code.

# Installation

You can use `thread_runner` in your project by adding the following to your `Cargo.toml` file at the dependencies section:

`thread_runner = "0.2.0"`

```
[dependencies]
thread_runner = "0.2.0"
```

Alternatively you can run the following command in the project directory:

```
cargo add thread_runner
```

This will add the latest version of the "thread_runner" to your Cargo.toml file


## Usage


This crate contains module, struct and function level documentations to help you
understand how various features employed within it work and how to use them

# Example 1

    use thread_runner::ThreadPool;

    let executor = ThreadPool::new(4);

    for val in 0..10 {
        executor.execute(move || println!("{}", val));
    }
    executor.join();

# Example 2

    use thread_runner::{AsyncRuntime, AsyncFlavor};
    use std::time::Duration;

    let rt = AsyncRuntime::new(AsyncFlavor::CurrentThread);

    // Spawn a future on the runtime.
    rt.execute(async {
         // Do some asynchronous work here...
    });

    // Poll a future on the runtime and block until it completes.
    let result = rt.poll(async {
        // Do some asynchronous work here and return a value...
        42
    });

    // Shut down the runtime after a specified timeout duration.
    rt.terminate(Duration::from_secs(1));



# Contributing

All contributions and suggestions are gladly welcome. Here are a few ways you can contribute:

- **Issue:** Report a bug or suggest an improvement by creating an issue.

- **Pull request:** Propose changes to the codebase by creating a pull request.

- **Documentation:** Contribute to documentation to help users understand how to use the software.

- **Testing:** Test the software and report any bugs or issues you find.

All contributions, large or small, are valuable and appreciated. Thank you for your interest in contributing to this project!



# License

This project is licensed under the MIT License - see the LICENSE file for details.
