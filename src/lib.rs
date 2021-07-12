mod front_of_house {
    pub mod hosting { // 公開
        pub fn add_to_waitlist() {} // 公開

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        mod back_of_house {


            fn fix_incorrect_order() {
                cook_order();
                super::serve_order(); // 相対パスの利用
            }

            fn cook_order() {}
        }

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // ← エラー解消

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // ← エラー解消

}
