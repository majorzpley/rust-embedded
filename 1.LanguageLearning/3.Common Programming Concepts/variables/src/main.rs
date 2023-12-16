fn main() {
    let mut x = 5;
    println!("The value of x is:{x}");
    //todo:不能为不可变量x赋予两次值
    x = 6;
    println!("The value of x is:{x}");
    //todo:shadowing
    let x = 5; //5
    let x = x + 1; //6

    {
        let x = x * 2; //12
        println!("The value of x in the inner scope is:{x}"); //12
    }
    println!("The value of x is:{x}"); //6
    //todo:shadowing和更改值类型不同的是：当我们再次使用关键字时，有效地创建一个新变量，可以更改值的类型，但重复使用相同的名称
    let spaces = "   "; //string type
    let spaces = spaces.len(); //number type
    // let mut spaces = "   "; //string type
    // spaces = spaces.len(); //number type
    
}
