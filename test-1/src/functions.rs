pub fn run() {
    greeting("Hello", "Nikhil");

    //Return
    println!("{}", add(5,5));

    //Closure
    let add_nums = |n1 : i32, n2 : i32| n1 + n2;
    println!("{}", add_nums(3, 3));
}

fn greeting(greet : &str, name : &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1 : i32, n2 : i32) -> i32 {
    n1 + n2
}