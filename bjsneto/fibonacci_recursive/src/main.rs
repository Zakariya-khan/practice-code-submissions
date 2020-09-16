fn fibonacci(number: i32) -> i32 {
    if number < 2 {
        return number;
    }
    return fibonacci(number - 1) + fibonacci(number - 2);
}

fn main() {
    for x in 0..=10 {
        println!("Fibonacci({}) = {}", x, fibonacci(x));
    }
}
