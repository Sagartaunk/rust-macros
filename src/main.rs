mod macros;
fn main() {
    println!(
        "The number is {} and times ten is {}",
        four!(),
        times_ten!(four!())
    );
}
