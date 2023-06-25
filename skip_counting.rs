// Simple utility to generate multiplication skip counting lists
// written in Rust.

// Example:
// 1 x 1 = 1
// 1 x 2 = 2
// ...
// 20 x 20 = 400

fn main() {
    // Iterate over a range, here 1 through 20.
    for n in 1..21 {
        println!(""); // Dirty line break. Prettify this later...
        let digit = n;
        for (_index, value) in (1..21).enumerate() {
            println!("{} x {} = {} ", digit, value, digit*value);
        }
    }
}
