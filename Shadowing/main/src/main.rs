use std::mem;

const MEANING_OF_LIFE :u8 = 42; // no fixed address

static mut Z:i32 = 133;

fn main() {

    println!("{}", MEANING_OF_LIFE);
    unsafe {
        println!("{}", Z);
    }


}