extern crate rand;

use std::io;
use std::num; //sqrt함수를 사용하기 위함... 어떻게 이 정보를 알 수 있을까?
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let dot1 = dot::newdot();
    let dot2 = dot::newdot();
    println!("{:?}", dot1);
    println!("{:?}", dot2);
    let dist1 = dot1.dist(&dot2);
    println!("두점 사이의 거리 : {}",dist1);

    let dist2 = dot1.diff(&dot2);
    println!("x와 y 차이 {:?}",dist2);
}
// 두점
#[derive(Debug)]
struct dot{
    x:i32,
    y:i32,
}
// 1. 두점 사이의 거리를 구하거나
// 2. x, y의 각각의 차이를 구하고싶음.
impl dot {
    fn newdot() -> dot{
        dot {
            x : rand::thread_rng().gen_range(1, 101),
            y : rand::thread_rng().gen_range(1,101)
        }
    }

    fn dist(&self, other: &dot) -> f32{
        let mut x = (&self.x - other.x).abs();
        let mut y = (&self.y - other.y).abs();
        x *= x;
        y *= y;
        ((x + y)as f32).sqrt()
    }

    fn diff(&self, other: &dot) -> dot{
        let  dot1 = dot {x : (self.x - other.x).abs() , y : (self.y - other.y).abs() };
        dot1
    }
}