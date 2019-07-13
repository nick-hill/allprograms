use std::io::stdin;

pub fn run() {

    println!("Write something");

    let mut var = String :: new();

    stdin().read_line( &mut var)
        .expect("Failed to read line");

    //Using text_io
    let i : String = read!("{}\n");

    println!("{}", i);

}