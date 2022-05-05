fn main() {
    // The [T; n] notation is used to create an array of n elements of type T in Rust

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [1234; 500];

    // Indexing starts at 0
    println!("first element of the array xs : {}", xs[0]);
    println!("second element of the array xs: {}", xs[1]);

    println!("first element of the array ys : {}", ys[0]);
    println!("second element of the array ys: {}", ys[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in xs array: {}", xs.len());


    // mutable array 
    let mut mutable_array: [i32; 100] = [111; 100];
    // updating element at index position 10
    mutable_array[10] = 1234;
    println!("Value of element at 10th index position in mutable_array after updation {}",mutable_array[10]);


}
