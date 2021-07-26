fn main() {
    println!("{:?}", listing_19_28![1, 2, 3]);
}


#[macro_export]
macro_rules! listing_19_28 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
