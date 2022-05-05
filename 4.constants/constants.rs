fn main() {
    //const key word makes a var const 
    const SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3;
    println!("{}" , SECONDS_IN_THREE_HOURS);

    // Any attempt to change the const variales will lead to errors
    // Below statement throws an error as constant variables can't be changed
    // SECONDS_IN_THREE_HOURS = 1000; 
     
}
    