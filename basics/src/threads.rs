use std::thread;

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