pub fn run()
{
    let mut numbers : Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);

    numbers.push(20);
    numbers.push(48);

    println!("{:?}", numbers);

    //Loop through vector
    for x in numbers.iter(){
        println!("{}", x);
    }
}