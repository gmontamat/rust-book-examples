fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n: u64 = 42;
    println!("F_{n} = {}", fibonacci(n));
}
