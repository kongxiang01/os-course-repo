#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("22301106 杨彬铁");
    for i in 1..=10 {
        let square = i * i;
        println!("{} ^ 2 = {}", i, square);
    }
    println!("Test OK!");
    0
}