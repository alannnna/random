// EXERCISE 5: The deliverable — a working guessing game engine.
// The game logic compiles but is broken. Fix the two bugs.
// Run: cargo test --bin ex05_final -p module_02_guesser

#[derive(Debug, PartialEq)]
enum Outcome {
    Win { attempts: usize },
    Loss { attempts: usize },
}

fn play(secret: u32, guesses: &[u32]) -> Outcome {
    for (i, &guess) in guesses.iter().enumerate() {
        if guess == secret {
            return Outcome::Win { attempts: i };
        }
    }
    Outcome::Loss { attempts: guesses.len() }
}

fn score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Win { attempts } => {
            if *attempts == 0 {
                0
                   // Fix play() first — then this branch becomes unreachable
            } else {
                100 / *attempts as u32
            }
        }
        Outcome::Loss { .. } => 0,
    }
}

fn main() {
    let outcome = play(42, &[10, 30, 42]);
    println!("Outcome: {:?}", outcome);
    println!("Score: {}", score(&outcome));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        assert_eq!(play(7, &[3, 9, 7]), Outcome::Win { attempts: 3 });
        assert_eq!(play(7, &[1, 2, 3]), Outcome::Loss { attempts: 3 });
        assert_eq!(play(7, &[7]), Outcome::Win { attempts: 1 });
    }

    #[test]
    fn test_score() {
        assert_eq!(score(&Outcome::Win { attempts: 1 }), 100);
        assert_eq!(score(&Outcome::Win { attempts: 4 }), 25);
        assert_eq!(score(&Outcome::Loss { attempts: 5 }), 0);
    }
}
