// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    my_macro!(42);
}

macro_rules! my_macro {
    ($value:expr) => {
        println!("Check out my macro!{}",$value);
    };
}
