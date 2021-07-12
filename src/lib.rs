// ↓ Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
