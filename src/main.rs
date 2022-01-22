extern crate rand;
use std::io::BufRead;
use std::{collections::HashMap, io};

use rand::{thread_rng, Rng};

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

impl TryFrom<char> for Placement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'g' => Ok(Placement::Correct),
            'y' => Ok(Placement::Misplaced),
            'b' => Ok(Placement::Absent),
            _ => Err(()),
        }
    }
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

impl<'a> TryFrom<&'a str> for PlacementInfo {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() != WORD_LENGTH {
            return Err(());
        }
        let mut placements = [Placement::Absent; WORD_LENGTH];
        for (i, c) in value.chars().enumerate() {
            placements[i] = Placement::try_from(c)?;
        }
        Ok(PlacementInfo { placements })
    }
}

fn placement(target: &str, guess: &str) -> PlacementInfo {
    let mut ret = PlacementInfo {
        placements: [Placement::Absent; WORD_LENGTH],
    };
    for (i, (gi, ti)) in guess
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

#[derive(Debug, Clone)]
struct Solver {
    sorted: Vec<String>,
}

impl Solver {
    fn make(words: &[String], frequencies: &HashMap<String, u64>) -> Self {
        let mut sorted = Vec::with_capacity(words.len());
        for word in words {
            sorted.push(word.clone());
        }
        sorted.sort_by_key(|x| frequencies.get(x).unwrap_or(&0));
        sorted.reverse();
        Solver {
            sorted,
        }
    }

    fn update(&mut self, guess: &str, info: PlacementInfo) {
        let mut new_sorted = self
            .sorted
            .iter()
            .filter(|x| consistent(x, guess, info))
            .map(|x| x.clone())
            .collect();
        std::mem::swap(&mut self.sorted, &mut new_sorted)
    }

    fn next_guess(&self) -> &str {
        &self.sorted[0]
    }
}

/// consistent checks if a word is consistent with a guess and its response.
fn consistent(word: &str, guess: &str, info: PlacementInfo) -> bool {
    word.chars()
        .zip(guess.chars())
        .take(WORD_LENGTH)
        .enumerate()
        .all(|(i, (wi, gi))| match info.placements[i] {
            // If the guess has placed a character correctly, then
            // we know what that character should be in the target
            Placement::Correct => wi == gi,
            // If the guess has a misplaced character, we know what the slot
            // should not contain
            Placement::Misplaced => wi != gi,
            // The character should be absent from the target as well
            Placement::Absent => !word.contains(gi),
        })
}

fn read_wordle_answers() -> Vec<String> {
    let file = include_str!("../data/wordle-answers.txt");
    file.lines().map(|x| x.to_owned()).collect()
}

fn read_frequencies() -> HashMap<String, u64> {
    let file = include_str!("../data/frequencies.csv");
    let mut out = HashMap::new();
    for line in file.lines() {
        let mut split = line.split(',');
        let word = split.next().unwrap();
        let count = u64::from_str_radix(split.next().unwrap(), 10).unwrap();
        out.insert(word.to_owned(), count);
    }
    out
}

fn print_placement(guess: &str, placement: &PlacementInfo) {
    for (i, char) in guess.chars().enumerate() {
        print!("\u{001b}[1m");
        match placement.placements[i] {
            Placement::Correct => print!("\u{001b}[42m{char}\u{001b}[0m"),
            Placement::Misplaced => print!("\u{001b}[43m{char}\u{001b}[0m"),
            Placement::Absent => print!("\u{001b}[40m{char}\u{001b}[0m"),
        }
    }
    println!();
}

fn play_interactive_wordle() -> io::Result<()> {
    println!("Guess a 5 letter word:");
    let words = read_wordle_answers();
    let mut rng = thread_rng();
    let target: &str = &words[rng.gen_range(0..words.len())];
    for maybe_guess in io::stdin().lock().lines() {
        let guess = maybe_guess?;
        let placement = placement(target, &guess);
        print_placement(&guess, &placement);
        if guess == target {
            println!("\ncongratulations!");
            break;
        }
    }
    Ok(())
}

fn guess_wordle() -> io::Result<()> {
    let answers = read_wordle_answers();
    let frequencies = read_frequencies();
    let mut solver = Solver::make(&answers, &frequencies);
    let mut guess = solver.next_guess().to_owned();
    println!("guess: {}", &guess);
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?;
        let info = match PlacementInfo::try_from(line.as_str()) {
            Ok(info) => info,
            Err(_) => continue
        };
        solver.update(&guess, info);
        guess = solver.next_guess().to_owned();
        println!("guess: {}", guess);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    guess_wordle()
}
