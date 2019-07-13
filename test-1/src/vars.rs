pub fn run() {

    let name = "Nick";
    let mut age = 22;

    println!("My name is {} and I am {} years old.", name, age);
    age = 23;
    println!("My name is {} and I am {} years old.", name, age);

    //Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    //Assign multiple vars

    let (my_name, my_age) = ("Nick", 23);
    println!("{} is {}.", my_name, my_age);
}