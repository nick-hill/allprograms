use std :: mem;

pub fn run() {

    let mut numbers : [i32; 5] = [0,1,2,3,4];

    println!("{:?}", numbers);

    numbers[1] = 15;

    //Array Length
    println!("{}", numbers.len());

    //Size of array
    println!("{}", mem::size_of_val(&numbers));
}
