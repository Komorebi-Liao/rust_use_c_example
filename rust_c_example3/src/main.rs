use ferris_says::say; // from the previous step
use libloading::{Library, Symbol};
use std::io::{stdout, BufWriter};

fn main() {
    unsafe {
        let lib = Library::new(r"./lib/hello_from_c.so").unwrap();

        let hello_from_c: Symbol<unsafe extern "C" fn()> = lib.get(b"hello_from_c").unwrap();

        hello_from_c();
    }
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
