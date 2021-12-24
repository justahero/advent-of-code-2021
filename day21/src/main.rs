use std::fmt::Display;

use itertools::Itertools;

trait Roll {
    fn roll(&mut self, time: u32) -> Vec<u32>;
}

struct DeterministicDice {
    pub start: u32,
    pub count: u32,
}

impl Default for DeterministicDice {
    fn default() -> Self {
        Self { start: 0, count: 0 }
    }
}

impl Roll for DeterministicDice {
    fn roll(&mut self, times: u32) -> Vec<u32> {
        let values = (0..times).map(|step| ((self.start + 1) % 100) + 1).collect_vec();
        self.start += times;
        values
    }
}

#[derive(Debug)]
struct Player {
    pub pos: u32,
    pub score: u32,
}

impl Player {
    pub fn new(start: u8) -> Self {
        Self { pos: start as u32, score: 0 }
    }

    /// Apply the rolled die, move position and return current score
    pub fn play(&mut self, die: &[u32]) -> u32 {
        let sum = die.iter().sum::<u32>();
        self.pos = ((self.pos - 1 + sum) % 10) + 1;
        self.score += self.pos;
        self.score
    }
}

#[derive(Debug)]
struct Game {
    pub player1: Player,
    pub player2: Player,
    pub roll_count: u32,
}

impl Game {
    pub fn new(player1: u8, player2: u8) -> Self {
        Self {
            player1: Player::new(player1),
            player2: Player::new(player2),
            roll_count: 0,
        }
    }

    /// Plays until one player wins, the returned tuple is (winner, loser)
    pub fn play(&mut self, mut dice: impl Roll) -> Option<(&Player, &Player)> {
        for _ in 1.. {
            let values = dice.roll(3);
            self.roll_count += 3;

            if self.player1.play(&values) >= 1000 {
                return Some((&self.player1, &self.player2));
            }

            let values = dice.roll(3);
            self.roll_count += 3;

            if self.player2.play(&values) >= 1000 {
                return Some((&self.player2, &self.player1));
            }
        }

        None
    }
}

fn main() {
    // Given input, player1 starts at 7, player2 starts at 3.
    let mut game = Game::new(7, 3);
    let dice = DeterministicDice::default();
    let (_winner, loser) = game.play(dice).unwrap();
    dbg!(loser.score * game.roll_count);
}

#[cfg(test)]
mod tests {
    use crate::{DeterministicDice, Game};

    #[test]
    fn test_deterministic_game() {
        let mut game = Game::new(4, 8);
        let dice = DeterministicDice::default();
        let (_winner, loser) = game.play(dice).unwrap();
        assert_eq!(739785, loser.score * game.roll_count);
    }
}
