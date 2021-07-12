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

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // 一部のみ公開
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // ← エラー解消

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // ← エラー解消

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    meal.seasonal_fruit = String::from("blueberries"); // ← 構造体を公開してもフィールドは自動的に公開されない

    // ↓ enumを公開すると、そのヴァリアントはすべて公開される。
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
