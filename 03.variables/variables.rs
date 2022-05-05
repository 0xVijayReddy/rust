fn main() {
    // Variables can be type annotated.
    let logical: bool = true;
    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    println!("{} {} {} ",logical,a_float,an_integer);


    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    println!(" {} {} ",default_float,default_integer);

    // A is variable is immuatable by default in Rust
    // But we can make a variable immutable by using mut keyword
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("value of mutable before changing is {}", mutable);
    mutable = 21;
    println!("value of mutable after changing is {}", mutable);
    
   
}

