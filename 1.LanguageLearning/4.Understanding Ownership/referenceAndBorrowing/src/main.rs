fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //todo:Mutable References可变引用
    let mut s = String::from("hello");
    change(&mut s);
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    //todo:创建多个可变引用
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    //todo:可变与不可变引用混合使用
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s;
    println!("{}", r3);

    //todo:Dangling Reference
}
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// fn dangle() -> String {
//     // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.