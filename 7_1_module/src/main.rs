extern crate module;    // Cargo.toml 파일 내 name을 적는다



// use 이름 가져오기

// 여러 요소 가져오기
use module::network::server::{connect, disconnect};
use TrafficLight::*;

enum TrafficLight{
    red,
    yellow,
    green,
}

fn main() {
    module::client::connect();  // 이름 선언 안 했을 때
    connect();  // 이름 영역 선언 시 : module::network::server::connect()

    let traffic_color = red;
}