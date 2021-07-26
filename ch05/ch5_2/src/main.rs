fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}


/*
❌ 相互に関係があるデータ（width,height）の関連性を示す構造がない
fn area(width: u32, height: u32) -> u32 {
    width * height
}
↓
❌ 多少構造的だが明確性に欠ける
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
↓
⭕ データの関連性を明確に示す構造がある */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}