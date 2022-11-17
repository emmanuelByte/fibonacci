use std::io;
fn main() {
    let mut prev: u32 = 0;
    let mut curr = 1;
    let mut next = String::new();
    let mut vec = vec![];
    println!("Fibonacci sequence:");
    println!("Enter the nth sequence");
    io::stdin()
        .read_line(&mut next)
        .expect("Failed to read line");
    let next: u32 = next.trim().parse().expect("Please type a number!");
    for _ in 0..next {
        let temp = prev;
        vec.push(prev);
        prev = curr;
        curr = temp + curr;
    }
    println!("\n{:?}", vec);
    println!("The {}th sequence is {}", next, vec[next as usize - 1]);
}
