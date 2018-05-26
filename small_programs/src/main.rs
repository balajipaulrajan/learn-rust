use std::io;

fn main() {
    fibonacci();
}

fn fibonacci() {
    println!("Enter the number");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse().expect("Expected a number");
    println!("you entered {}", n);
    if n < 1 {
        println!("number should be greater than 0");
        return;
    }
    let i = fib_internal(n);
    println!("{}", i);
}
fn fib_internal(n: i64) -> i64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fib_internal(n - 1) + fib_internal(n - 2)
    }
}
