fn main() {
    let mut s = String::from("hello world");
    //s.clear(); // this empties the String, making it equal to ""
    let word = first_word(&s); // word will get the value 5

    // s.clear(); // 这里再次引用s会导致编译错误

    println!("the first word is: {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
