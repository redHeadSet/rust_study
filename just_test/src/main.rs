extern crate rand;

// use std::io;
// use std::num; //sqrt함수를 사용하기 위함... 어떻게 이 정보를 알 수 있을까?
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let dot1 = Dot::new_dot();
    let dot2 = Dot::new_dot();
    println!("{:?}", dot1);
    println!("{:?}", dot2);
    let dist1 = dot1.dist(&dot2);
    println!("두점 사이의 거리 : {}",dist1);

    let dist2 = dot1.diff(&dot2);
    println!("x와 y 차이 {:?}",dist2);
}
// 두점
#[derive(Debug)]
struct Dot{
    x:i32,
    y:i32,
}
// 1. 두점 사이의 거리를 구하거나
// 2. x, y의 각각의 차이를 구하고싶음.
impl Dot {
    fn new_dot() -> Dot{
        Dot {
            x : rand::thread_rng().gen_range(1, 101),
            y : rand::thread_rng().gen_range(1,101)
        }
    }

    fn dist(&self, other: &Dot) -> f32{
        let x = (&self.x - other.x).abs();
        let y = (&self.y - other.y).abs();
        let bb:f32 = (x*x+y*y) as f32;
        bb.sqrt()
        // ((x + y)as f32).sqrt()
    }

    fn diff(&self, other: &Dot) -> Dot{
        let  dot1 = Dot {x : (self.x - other.x).abs() , y : (self.y - other.y).abs() };
        dot1
    }
}
