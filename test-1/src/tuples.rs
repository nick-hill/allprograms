pub fn run() {

    let person : (&str, &str, i32) = ("Nikhil", "Delhi", 23);

    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}