pub fn fact() {

    println!("Enter a number: ");
    let var : i32 = read!();

    println!("The factorial of {} is {}.",var, factorial(var));

}

fn factorial (val : i32) -> i32 {
    if val == 0 {
        return 1;
    }
    return val * factorial(val - 1);
}