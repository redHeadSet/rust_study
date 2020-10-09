struct User{
    username: String,   // &str을 쓰지 않는 이유는 lifetime 때문이다 (추후 공부 예정)
    email: String,
    sign_in_count: u32,
    active: bool
}

#[derive(Debug)]
struct Rect{        // 위와 같은 #derive 디버깅 정보 출력과 같은 어노테이션은 구조체 바로 위에서 해주어야 한다.
    length: u32,
    width: u32,
}

impl Rect{  // Rect 구조체 내 관련 메소드는 이와 같이 정의한다 impl~
    fn size(&self) -> u32 {
        self.length * self.width
    }   // 세미콜론이 없음에 주의

    fn compare_inside(&self, compare_rect: &Rect) -> bool {     // 여러 인자 전달 시 해당 방법으로 사용
        self.length >= compare_rect.length && self.width >= compare_rect.width
    }

    fn square(length: u32) -> Rect{ // 메소드 중 인자가 &self 값이 없다면 연관함수라 칭함
        let square_rect = Rect{ length, width:length };
        square_rect
    }
}

fn main() {
    let user1 = User{
        username: String::from("hjjung"),
        email: String::from("stikfascube@gamil.com"),
        sign_in_count: 1,
        active: true
    };

    user_prints(&user1);

    // let user2 = user1;
    let user2 = User{
        username: String::from("jhj"),
        active: false,
        ..user1 // 나머지 데이터를 user1과 동일하게 맞춤 (순서는 상관없는 걸로 보임)
    };

    user_prints(&user2);

    let rect1 = Rect{ length: 50, width: 30 };

    println!("rect info1 : {:?}", rect1);       // 서로 출력 형태가 다름 ( # 없음 )
    println!("\n\nrect info2 : {:#?}", rect1);  // 서로 출력 형태가 다름 ( # 있음 )

    println!("rect size fn   : {}", get_rect_size(&rect1));  // 일반 외부 함수 사용하여 출력
    println!("rect size impl : {}", rect1.size());      // 내부 메소드 사용하여 출력

    let rect2 = Rect::square(30);   // 연관함수 호출
    println!("rect2 info : {:#?}", rect2);
    println!("rect2 inside rect1 ? : {}", rect1.compare_inside(&rect2));
}

fn user_prints(input_user: &User) {
    println!("user contanins : ");
    println!("user.username      : {}", input_user.username);
    println!("user.email         : {}", input_user.email);
    println!("user.sign_in_count : {}", input_user.sign_in_count);
    println!("user.active        : {}", input_user.active);
}

fn get_rect_size(input_rect: &Rect ) -> u32 {
    input_rect.length * input_rect.width
}