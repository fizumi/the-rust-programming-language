
// https://doc.rust-lang.org/std/rc/struct.Rc.html

// cargo test -- --nocapture rc
// cargo test rc::tests 2>/dev/null
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    #[test]
    fn clone() {
        let five = Rc::new(5);
        {
            let _also_five = Rc::clone(&five); // Box::new は所有権を奪うが、 Rc::clone なら奪わずに参照を得ることが出来る
            println!("strong_count five = {}", Rc::strong_count(&five));
            println!("strong_count _also_five = {}", Rc::strong_count(&_also_five));
            assert_eq!(2, Rc::strong_count(&five));
        }

        assert_eq!(1, Rc::strong_count(&five));

        {
            let _also_five = Rc::clone(&five);
            let _also_five2 = Rc::clone(&_also_five);
            println!("strong_count five = {}, value = {}", Rc::strong_count(&five), five);
            println!("strong_count _also_five2 = {}, value = {}", Rc::strong_count(&_also_five2), _also_five2);
            assert_eq!(3, Rc::strong_count(&five));
        }
    }
    #[test]
    fn get_mut() {
        let mut x = Rc::new(3);
        *Rc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);
        {
            let x2 = Rc::clone(&x);
            assert_eq!(Rc::get_mut(&mut x), None); // 強参照が２つ以上ある場合は可変参照を作成できない
        }

    }
}