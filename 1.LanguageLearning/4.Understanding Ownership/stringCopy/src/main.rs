fn main() {
    let s1 = String::from("hello");
    //todo:heap data does get copied
    let s2 = s1.clone();
    println!("s1 = {},s2 = {}", s1, s2);

    //todo:Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); //s's value moves into the function...
                        //...and so is no longer valid here
    let x = 5; //x comes into scope
    makes_copy(x); //x would move into the function,
                   //but i32 is copy ,so it's okay to still use x afterward

    //todo:Return Values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    //todo:返回元组Rust does let us return multiple values using a tuple
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
//memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
