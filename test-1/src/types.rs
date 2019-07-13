pub fn run() {

    //Default i32
    let x = 1;

    //Default f64
    let y = 2.5;

    //Explicit dec
    let z : i64 = 3424234322;

    //Max size
    println!("{}", std::i128::MAX);

    //Boolean
    let is_active = true;

    let is_greater = 10 < 5;

    let a1 = '\u{1F600}';


    println!("{:?}",( x, y, z, is_active, is_greater, a1));

}