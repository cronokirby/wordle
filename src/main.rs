/// This represents the information we receive for each character in a guess.
///
/// This information is what we use to then produce our next guess.
#[derive(Debug)]
enum Placement {
    /// The character is at the right spot.
    Correct,
    /// The character is at the wrong spot, but in the word nonetheless.
    Misplaced,
    /// The character is not in the word.
    Absent,
}

fn main() {
    println!("Hello, world!");
}
