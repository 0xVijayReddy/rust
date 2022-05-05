use std::collections::HashMap;

fn main() {
    // initialize the HashMap
    let mut contacts = HashMap::new();

    // inserting key-value pairs to the contacts hashmap 
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // printing the key values in HashMap using iter method

    for (key, val) in contacts.iter() {
        println!("{} {}", key, val);
    }

    // Use contains_key(& key) to check the key present or not 
    if contacts.contains_key(&"Daniel"){
        println!("Calling Daniel ....");
    }
    else{
        println!("Can't find Daniel's contact");
    }

    // length of HashMap using len() methods
    println!("Length of contacts HashMap is {}",contacts.len());

    // remove a key from HashMap using remove() method
    contacts.remove(&"Daniel");
    println!("HashMap after removing Daniel's contact");
    for (key, val) in contacts.iter() {
        println!("{} {}", key, val);
    }

    // Using get( & key) method  for getting the value for a particular key
    let phone_number = contacts.get(&"Katie");
    println!("Katie's phone number is {:?}",phone_number);





}
