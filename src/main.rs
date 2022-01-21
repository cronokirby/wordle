/// This represents the information we receive for each character in a guess.
///
/// This information is what we use to then produce our next guess.
#[derive(Clone, Copy, Debug)]
enum Placement {
    /// The character is at the right spot.
    Correct,
    /// The character is at the wrong spot, but in the word nonetheless.
    Misplaced,
    /// The character is not in the word.
    Absent,
}

/// WORD_LENGTH is the number of characters in each word.
const WORD_LENGTH: usize = 5;

/// This gives us all the information we receive after a guess.
///
/// For each character in our guess, we get information about its placement.
#[derive(Clone, Copy, Debug)]
struct PlacementInfo {
    placements: [Placement; WORD_LENGTH],
}

fn placement(target: &str, guess: &str) -> PlacementInfo {
    let mut ret = PlacementInfo {
        placements: [Placement::Absent; WORD_LENGTH],
    };
    for (i, (ti, gi)) in guess
        .chars()
        .zip(target.chars())
        .enumerate()
        .take(WORD_LENGTH)
    {
        if ti == gi {
            ret.placements[i] = Placement::Correct;
        } else if target.contains(gi) {
            ret.placements[i] = Placement::Misplaced;
        }
    }
    ret
}

fn main() {
    println!("Hello, world!");
}
