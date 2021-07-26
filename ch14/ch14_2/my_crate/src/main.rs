extern crate art;

// 再エクスポートなしの場合
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// 再エクスポートありの場合
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}