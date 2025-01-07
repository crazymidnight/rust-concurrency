use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
    })
    .join()
    .unwrap();

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

    static X: [i32; 3] = [1, 2, 3];
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));

    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));

    let a = Arc::new([1, 2, 3]);
    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    dbg!(a);

    let a = &Cell::new(42);
    f_cell(a, a);

    thread::sleep(Duration::from_secs(5));
}

fn func() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn f_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("Hi");
    }
}
