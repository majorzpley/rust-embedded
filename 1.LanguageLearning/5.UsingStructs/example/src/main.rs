#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square.",
        area1(width1, height1)
    );

    //todo:Refactoring with Tuples,使用元组
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square.", area2(rect1));

    //todo:Refactoring with Structs: Adding More Meaning
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square.", area3(&rect1));
    dbg!(&rect1);
}
fn area1(width: u32, height: u32) -> u32 {
    width * height
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
