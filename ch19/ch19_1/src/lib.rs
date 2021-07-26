
// ポインタの"生成"は safe （Listing 19-1）
fn creating_raw_pointers_from_references() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

// メモリの任意の箇所を指す生ポインタの生成 （Listing 19-2）
fn creating_a_raw_pointer_to_an_arbitrary_memory_address() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

/*
cargo test -- --nocapture 2>/dev/null
*/
// Listing 19-3
#[test]
fn dereferencing_raw_pointers_within_an_unsafe_block() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// unsafe 関数の呼び出しは unsafe ブロックが必要
unsafe fn dangerous() {}

fn call_dangerous(){
    unsafe {
        dangerous();
    }
}

// Listing 19-4:
fn using_the_safe_split_at_mut_function() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); // 内部的に unsafe を利用している

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// スライスの異なる部分を借用することは、根本的に大丈夫だが、
// コンパイラはこれを知れるほど賢くない
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..])
// }
// Listing 19-6
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    use std::slice;
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using extern Functions to Call External Code （Listing 19-8）
// declaring an extern function
extern "C" {
    fn abs(input: i32) -> i32;
}

/*
cargo test another_language -- --nocapture 2>/dev/null
*/
#[test]
fn extern_function_defined_in_another_language() {
    // calling an extern function
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 可変で静的な変数にアクセスし変更することは unsafe
static mut COUNTER: u32 = 0;

fn writing_to_a_mutable_static_variable_is_unsafe(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn reading_from_a_mutable_static_variable_is_unsafe() {
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}


// Listing 19-11: Defining and implementing an unsafe trait
// unsafeなトレイトの実装
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}