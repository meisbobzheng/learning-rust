fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let x = 5;
    let x = x + 1;
    
    // shadowing w/ inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let tuple: (i32, f64, i32) = (500, 6.4, 1);
    let (_x, y, _z) = tuple;
    println!("The value of y is: {}", y);

    let immutable_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", immutable_array);

    let mut mutable_array: [i32; 5] = [1, 2, 3, 4, 5];
    mutable_array[0] = 10;
    println!("The value of a is: {:?}", mutable_array);
}
