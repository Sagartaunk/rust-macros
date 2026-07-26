mod macros;
fn main() {
    println!(
        "The number is {} and times ten is {}",
        four!(),
        times_ten!(four!())
    );
    let hash = to_hashmap!(
        "something" => "another Thing",
        "not_something" => "something else"
    );
    println!("{:?}", hash);
}
