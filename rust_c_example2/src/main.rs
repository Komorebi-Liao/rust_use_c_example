/*
 * @Author: lzy
 * @Date: 2023-06-07 10:56:39
 * @LastEditors: lzy
 * @LastEditTime: 2023-06-07 11:18:38
 * @FilePath: \rust_use_c_example1\src\main.rs
 * @Description:
 *
 */
extern crate libc;

extern "C" {
    fn hello_from_c();
    fn add(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn main() {
    let input_a = 4;
    let input_b = 8;
    let output = unsafe { add(input_a, input_b) };
    unsafe { hello_from_c() };
    println!("add from c:{} + {} = {}", input_a, input_b, output);
}
