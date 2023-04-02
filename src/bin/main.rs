use thread_runner::{AsyncFlavor, AsyncRuntime};

fn main() {
    let runtime = AsyncRuntime::new(AsyncFlavor::AllThreads);
    let res = runtime.poll(charge());
    println!("{}", res)
}

pub async fn charge() -> i32 {
    2
}
