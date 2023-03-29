# ThreadRunner

`thread_runner` is a Rust library for executing closures concurrently using a pool of worker threads inspired by the java `ExecutorService` package. It currently contains:
- execs:
A module containing the ExecutorService

The main benefit is the guarantee that calls to `execute` and `join` will never panic.

## Installation

You can use `thread_runner` in your project by adding the following to your `Cargo.toml` file at the dependencies section:

`thread_runner = "0.1.0"`

```
[dependencies]
thread_runner = "0.1.0"
```

Alternatively you can run the following command in the project directory:
```
cargo add thread_runner
```

This will add the latest version of the "thread_runner" to your Cargo.toml file and download it to your local machine.


## Usage

To use the `ExecuterService`, import it from the `execs` module and initialize it with a specified number of workers:

```rust
use thread_runner::execs::ExecutorService;

let executor = ExecutorService::new(4); // create an executor with 4 worker threads
```

Then, submit closures to the executor using the execute method:

```
executor.execute(|| {
    // closure to execute concurrently
});
```

To wait for all submitted closures to finish executing, call the join method:

```
executor.join();
```

# Example

```
use thread_runner::execs::ExecutorService;

let executor = ExecutorService::new(4); // create an executor with 4 worker threads

for val in 0..1000 {
    executor.execute(move || {
        println!("{}", val);
    });
}

executor.join();

```

# License
This project is licensed under the MIT License - see the LICENSE file for details.