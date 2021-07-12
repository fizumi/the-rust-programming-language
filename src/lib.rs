mod front_of_house {
    pub mod hosting { // 公開
        pub fn add_to_waitlist() {} // 公開

        fn seat_at_table() {}
    }
}

// use crate::front_of_house::hosting; // ✔OK
// use self::front_of_house::hosting; // ✔OK
use front_of_house::hosting; // ✔OK
// use crate::front_of_house::hosting::add_to_waitlist; // ⚠ 文法は OK だが 慣例と異なる

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // add_to_waitlist(); // ⚠ 文法は OK だが 慣例と異なる
}
