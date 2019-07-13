pub fn b_sort() {
    println!("Enter number of elements: ");
    let var: usize = read!();

    let mut vec = Vec::new();
    let mut temp : usize;

    let mut val: usize;

    for _x in 0..var {
        val = read!();
        vec.push(val);
    }

    for x in 0..var{
        for y in 0..var - 1 {
            if vec[y] > vec[y + 1] {
                temp = vec[y];
                vec[y] = vec[y + 1];
                vec[y + 1] = temp;
            }
        }
    }
    println!("{:?}", vec);
}