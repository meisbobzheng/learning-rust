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

    //inline if statement
    let condition = true;
    let number = if condition {10} else {20};
    println!("The value of number is: {}", number);

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // loop labels
    counter = 0;
    'counting_up: loop {
        println!("count = {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    // while loop
    while counter != 0 {
        println!("{}!", counter);
        counter -= 1;
    }

    // for loop
    for element in immutable_array {
        println!("The value is: {}", element);
    }
}
