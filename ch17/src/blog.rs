// ステートパターン
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// ⭕ Postのメソッドは、種々の振る舞いについては何も知りません（状態と遷移のカプセル化）
//     ⇒ PostのメソッドとPostを使用する箇所で、match式が必要になることはない
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// ⭕ 新しい状態を追加する場合、新しい構造体を追加し、その1つの構造体にトレイトメソッドを実装するだけでいい
//     ⇒ 拡張して機能を増やすことが容易
// ❌ 状態が状態間の遷移を実装しているので、状態の一部が密に結合した状態になってしまう
// ❌ 「オブジェクト安全性」を侵害しないために、selfを返すデフォルト実装は不可
//     ⇒ 各メソッドで self を返すコードが重複する
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}