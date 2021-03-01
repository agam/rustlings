// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let mut myvec : Vec<u8> = Vec::new();
    for i in 0..150 {
        myvec.push(i)
    }
    let boxed_slice: Box<[u8]> = myvec.into_boxed_slice();

    let a: &[u8] = &*boxed_slice;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
