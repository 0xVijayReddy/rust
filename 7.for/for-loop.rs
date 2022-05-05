fn main() {
    // basic for loop
    // A for expression extracts values from an iterator, looping until the iterator is empty.
    for i in 0..10{
        println!("{}",i);
    }

    
    // A loop expression denotes an infinite loop.
    // A loop expression repeats execution of its body continuously 
    // until it encounters break statement or return statement

    let mut i :u32 = 1;
    loop{

        if i>=30{
            break;
        }
        else if i%15==0 {
            println!("FizzBuzz");
        }
        else if i%3==0 {
            println!("Fizz");
        }
        else if i%5==0{
            println!("Buzz");
        }
        
        i = i+1;
    }
   
}