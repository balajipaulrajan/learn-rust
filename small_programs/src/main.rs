use std::io;

fn main() {
    fibonacci();
    fahrenheit_to_celsius();
    celsius_to_fahrenheit();
    struct_sample();
    struct_impl_sample();
}

fn fibonacci() {
    println!("Enter the number for finding the fibonocci sequence");
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

fn fahrenheit_to_celsius() {
    println!(" Enter the temperature in fahrenheit");
    let n = read_input_number();
    let c = (n - 32.0) / 1.8;
    println!("Temperature in celsius is {:.2}", c);
}
fn celsius_to_fahrenheit() {
    println!(" Enter the temperature in celsius");
    let n = read_input_number();
    let f = (n * 1.8) + 32.0;
    println!("Temperature in fahrenheit is {:.2}", f);
}
fn read_input_number() -> f64 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read");
    let n = n.trim().parse().expect("Expected a number");
    n
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn struct_sample() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("rect is {:?}", rect);
    let area = rectangle_area(&rect);
    println!("area of rectangle is {}", area);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn struct_impl_sample() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("Area of rectangle is {}", rect.area());
    let rect1 = Rectangle {
        width: 4,
        height: 6,
    };
    println!("rect1 fits into rect - {}", rect.can_hold(rect1));
    let sq = Rectangle::square(10);
    println!("Area of square is {}", sq.area());
}
