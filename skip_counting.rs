// Simple utility to generate multiplication skip counting lists
// written in Rust.

// Example:
//  1 ×  1 =   1
//  1 ×  2 =   2
// ...
// 20 × 20 = 400

fn main() {
    // Use a constant for the range limit to make it easier to adjust
    const LIMIT: i32 = 20;

    // Iterate over the outer loop (1 to LIMIT)
    for digit in 1..=LIMIT {
        // Print a separator line before each table, but not before the first one
        if digit > 1 {
            println!();
        }

        // Inner loop for multiplication
        for value in 1..=LIMIT {
            // Use string formatting for cleaner output
            println!("{:2} × {:2} = {:3}", digit, value, digit * value);
        }
    }
}
