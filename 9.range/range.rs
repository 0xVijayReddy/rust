fn main() {
    //basic for loop using range iterator
    println!("\nBasic for loop");
    for i in 1..10 {
        println!("i={} " , i ) ;
    }
    println!("\nfor loop with range that is inclusive on both ends");
    //a..=b can be used for a range that is inclusive on both ends.
    for i in 1..=10 {
        println!("i={} " , i ) ;
    }
    
    // for loop using array as an iterator
    println!("\nfor loop using array as an iterator");
    let langs = ["Java","Go","clang","Python","Solidity"];
    for lang in langs {
        println!("{} " , lang ) ;
    }
}