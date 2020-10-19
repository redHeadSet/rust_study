
fn main() {
    let length1 = 50;
    let width1 = 30;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(length1, width1)
    // );
    
    let rect1 = Rectangle {length: 50, width: 30}; 
    println!("The area of the rectangle is {} 상자 픽셀:",area(&rect1));
    println!("{:?}",rect1);
    //메소드 이용
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect3 = Rectangle::square(3);
    println!("{:?}",rect3);
}

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

#[derive(Debug)]
struct Rectangle {
    length : u32,
    width: u32,
}
 //메소드 이용
 //impl : 구현 블록
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width //메소드 문법
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
    }

    //연관 함수 (associated functions)
    // self 파리미터를 갖지 않은 함수 : 함께 동작 할 인스턴스를 가지고 있지 않음.
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}