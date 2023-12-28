fn fib(num: u32) -> u32 {
    if num == 1 {
        return 1;
    }
    return num * fib(num - 1);
}

fn main() {
    let fib_num = fib(3);
    println!("Fibonacci number is {fib_num}");
}
