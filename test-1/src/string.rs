pub fn run() {

    let mut hello = String::from("HELLO ");

    println!("Length: {}", hello.len());

    //Push a character
    hello.push('H');

    println!("{}", hello);

    //Push a string
    hello.push_str(" HEY NICK");

    println!("{}", hello);

    //For capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Contains
    println!("Contains word 'HEY' {}", hello.contains("HEY"));

    //Replace
    println!("Replace word 'HEY' {}", hello.replace("HEY", "HOLA"));

    //Loop by whitespace

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //String with capacity
    let mut s = String::with_capacity(20);

    s.push('a');
    s.push('b');
    s.push('c');

    println!("{}", s);

    //Assertion testing
    // assert_eq!(2, hello.capacity());

}