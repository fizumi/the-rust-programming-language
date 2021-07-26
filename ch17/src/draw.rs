extern crate gui;
use gui::{ Draw, Screen };

// Listing 17-7: A Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// Listing 17-8: Another crate using gui and implementing the Draw trait on a SelectBox struct
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}


// Listing 17-9: Using trait objects to store values of different types that implement the same trait
// `gui` から `generics_and_trait_bounds` を再エクスポートすると以下のコードはエラーとなる
fn listing_17_9() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Listing 17-10: Attempting to use a type that doesn’t implement the trait object’s trait
// fn listing_17_10() {
//     let screen = Screen {
//         components: vec![Box::new(String::from("Hi"))], // ❌ the trait `draw::gui::Draw` is not implemented for `std::string::String`
//     };
//     screen.run();
// }