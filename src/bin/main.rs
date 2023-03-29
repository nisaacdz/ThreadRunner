use executors::ExecuterService;
fn main() {
    let ex = ExecuterService::new(10);

    for _ in 0..100 {
        ex.execute(|| do_something());
    }

    ex.join();
}

fn do_something() {
    for i in 0..10 {
        print!("value = {}; ", i);
    }
    println!()
}
