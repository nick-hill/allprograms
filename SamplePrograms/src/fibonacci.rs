pub fn fibo() {
    println!("Enter number: ");
    let var : i32 = read!();


    for i in 0..var
        {
            print!("{}", fib(i));
            print!(" ");
        }
}

fn fib(n : i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib (n -2);
}