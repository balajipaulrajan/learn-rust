use std::collections::HashMap;
fn main() {
    let mut v = vec![3, 2, 1, 4, 5];
    println!("{}", mean(&v));
    v.sort();
    println!("{}", median(&v));
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let count = v.len();
    for i in v.iter() {
        sum = sum + i;
    }
    (sum as f64) / (count as f64)
}

fn median(v: &Vec<i32>) -> i32 {
    let length = v.len();
    let centre = if length % 2 == 0 {
        (length) / 2
    } else {
        (length + 1) / 2
    };
    v[centre-1]
}
