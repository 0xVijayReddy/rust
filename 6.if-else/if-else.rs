fn main() {

    // Here's a basic example.
    if 7%2 == 0 {
        println!("7 is even");
    } else {
        println!("7 is odd");
    }

    // You can have an `if` statement without an else.
    if 8%4 == 0 {
        println!("8 is divisible by 4");
    }


    // if-else block with else if statement
    // else if block can be used if need to check for multiple if conditions

    let num = 9;
    if num < 0 {
        println!("{} is negative",num);
    } else if num < 10 {
        println!("{} has 1 digit",num);
    } else {
        println!("{} has multiple digits",num);
    }
}
