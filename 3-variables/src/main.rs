fn main() {
    // the following is shadow example
    let x = 5;

    // the previous x is shadow
    let x = x + 1;

    {
        let x = x * 2;
        // x is 12
        println!("The value of x in the inner scope is: {x}");
    }// shadow end at this scope

    // x is 6
    println!("The value of x is: {x}");

    // Rust doesn’t care where you define your functions, 
    // only that they’re defined somewhere in a scope that can be seen by the caller.
    reuse_name();

    statement_ret();

    println!("plus one {}", plus_one(12));

    control_flow();

    loop_case();

    nested_loop();

    while_case();

    for_case();
}

fn reuse_name() {
    // reuse the same name with different data type
    // immutable variable is okay
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // reuse the same name with different data type
    // mutable variable is not
    //
    // let mut spaces = "   ";
    // spaces = spaces.len();
}

fn statement_ret() {
    // "let y = 1;" statement not return any value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let condition = true;

    // Rust wouldn’t be able to do that 
    // if the type of number was only determined at runtime
    // let number = if condition { 5 } else { "six" };


    let number = if condition { 5 } else { 18 };

    println!("The value of number is: {number}");
}

fn loop_case() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_case() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_case() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}