pub mod trait_bound;

// 트레잇 정의 : Summarizable
pub trait Summarizable {
    // 트레잇 동작 정의
    fn summary(&self) -> String;

    // 트레잇 동작 정의 시, 기본 처리를 미리 넣어둘 수도 있다
    fn default_summary(&self) -> String {
        String::from("default summary")
    }
}
// 위 트레잇을 pub 로 설정하지 않아도 된다
// Summarizable 트레잇을 다른 크레이트에 부여하고 싶다면, pub 로 설정해야 한다
// 이 경우, 외부 트레잇도 공개가 되어있다면 역시 내부에서 사용하는 타입에 부여 가능하다.
// 한 가지 예외로, 외부 타입에 대한 외부 트레잇을 부여하는 것은 불가능하다.
// -> 고아 규칙에 의해 제한된다

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 트레잇 동작 구현 - NewsArticle 에 대해서만 동작
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})",
                self.headline,
                self.author,
                self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 트레잇 동작 구현 - Tweet 에 대해서만 동작
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}