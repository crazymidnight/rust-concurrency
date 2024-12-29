use std::thread;

fn main() {
    let t1 = thread::spawn(func);
    let t2 = thread::spawn(func);
    println!("Hello fron the main thread");
    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();

    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("average: {average}");
    thread::Builder::new().spawn(func).unwrap().join().unwrap();

    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}

fn func() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
