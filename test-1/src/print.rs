pub fn run(){

    println!("Hello from the main file");

    //Basic Formatting
    println!("{} is from {}", "Nikhil", "Delhi");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Nikhil", "Delhi", "Code");

    //Named Arguments
    println!("{name} likes to watch {sport}", name = "Nikhil", sport = "UFC");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "Hey"));

    //Basic Maths
    println!("10 + 10 = {}", 10+10);

}