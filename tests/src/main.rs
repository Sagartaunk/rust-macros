use rust_macros::test_find_line;

fn main() {
    println!("The line number for this laptop is {}", test_find_line!());
}
