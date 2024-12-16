// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


#[macro_use]  // 方法1，方法2：直接macros::xxx!()，方法3，新建Cargo.toml #[macro_use] extern crate 方法4，使用#[macro_export]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    use macros::*;
    my_macro!();
}
