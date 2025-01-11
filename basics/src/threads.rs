use std::thread;
use std::io::{self, Read, Write};
use std::sync::Arc;

// Numbers cannot be used outside of the thread, as the lifetime needs 
// to be static, and thus moved into the thread. 
pub fn one(){
    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move ||{
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len 
    }).join().unwrap();

    println!("Average: {:?}", t);
}

// We can borrow numbers here in the scoped thread, as none of the threads in scope
// can outlive the scope. Therefore, we can reference anything that outlives the scope
// such as numbers. 
pub fn scoped_thread(){
    let numbers = vec![1,2,3];
    thread::scope(|s|{
        s.spawn(|| {
            println!("{}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers{
                println!("{}", n);
            }
        });
    });

    let y = numbers;
}

// Statics are never owned by a single thread, because they belong to the entire program.
pub fn _static(){
    static X: [i32; 3] = [1,2,3];
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}

// Share ownership by leaking an allocation
// We release ownership of a Box, promising to never drop it
// This box will live forever, allowing it to be borrowed by any thread
// for aslong as the program runs
pub fn leak(){
    let x: &'static [i32; 3] = Box::leak( // Takes Box<T> and leaks 
                                          // memory. Returns reference to inner data &'static T, 
                                          // prevents deallocation
        Box::new([1,2,3]) // The Box puts the array on the heap
    );
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));

    println!("Please input your string");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = Box::new(input.trim().to_string());
    // String available until program exits 
    let string: &'static String = Box::leak(Box::new(input));

    dbg!(string);
}

// Atomically Reference Counted, smart pointer lives on stack and points to heap memory.
// Once all references go out of scope, drop and deallocate data. 
// Pointer lives on the stack as its lifetime is managed, however heap is managed
// by us. 
pub fn arc(){
    let a = Arc::new([1,2,3]);
    let b = Arc::clone(&a); 
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

// Here, b cant refer to a as b is being borrowed as a mut, whilst a is not.
// We cannot borrow data as mutable and immutable at the same time. 
pub fn data_race(a: &i32, b: &mut i32){
    let before = *a;
    *b+=1;
    let after = *a;
    if before != after{
        println!("This can never happen");
    }
}