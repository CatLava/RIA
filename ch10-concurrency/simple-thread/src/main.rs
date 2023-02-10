use std::{thread, time};


fn main() {
    println!("Hello, world!");
    let start = time::Instant::now();

    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler.join().unwrap();

    let finish = time::Instant::now();
    println!("{}", finish.duration_since(start));
}
