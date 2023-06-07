/*
 * @Author: lzy
 * @Date: 2023-06-07 10:57:17
 * @LastEditors: lzy
 * @LastEditTime: 2023-06-07 11:10:06
 * @FilePath: \rust_use_c_example1\build.rs
 * @Description:
 *
 */
extern crate cc;

fn main() {

    cc::Build::new()
        .file(r"lib/hello_from_c.c")
        .compile("hello_from_c.so");
    cc::Build::new().file(r"lib/add.c").compile("add.so");
    
    // static
    // cc::Build::new()
    //     .file(r"lib\hello_from_c.c")
    //     .compile("hello_from_c.a");
    // cc::Build::new().file(r"lib\add.c").compile("add.a");
}
