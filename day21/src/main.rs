use std::{borrow::BorrowMut, collections::HashMap};

use itertools::Itertools;

trait Roll {
    fn roll(&mut self, time: u32) -> Vec<u32>;
}

struct DeterministicDice {
    pub start: u32,
}

impl Default for DeterministicDice {
    fn default() -> Self {
        Self { start: 0 }
    }
}

impl Roll for DeterministicDice {
    fn roll(&mut self, times: u32) -> Vec<u32> {
        let values = (1..=times)
            .map(|step| ((self.start + step - 1) % 100) + 1)
            .collect_vec();
        self.start += times;
        values
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Player {
    pub pos: u32,
    pub score: u32,
}

impl Player {
    pub fn new(start: u8) -> Self {
        Self {
            pos: start as u32,
            score: 0,
        }
    }

    pub fn roll(&mut self, dice: u32) -> u32 {
        self.pos = ((self.pos - 1 + dice) % 10) + 1;
        self.score += self.pos;
        self.score
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Game {
    pub players: [Player; 2],
}

impl Game {
    pub fn new(player1: u8, player2: u8) -> Self {
        Self {
            players: [Player::new(player1), Player::new(player2)],
        }
    }

    /// Plays until one player wins
    pub fn play1(&mut self, mut dice: impl Roll) -> u32 {
        let mut roll_count = 0_u32;

        for index in (0..self.players.len()).cycle() {
            let sum = dice.roll(3).iter().sum();
            self.players[index].roll(sum);
            roll_count += 3;

            if self.players[index].score >= 1000 {
                break;
            }
        }

        // calculate result using score of loser
        let score = self.players.iter().map(|p| p.score).min().unwrap();
        score * roll_count
    }

    pub fn play2(&mut self) -> u128 {
        let mut wins = [0_u128, 0];
        let mut games = HashMap::from([(
            Game {
                players: self.players.clone(),
            },
            1u128,
        )]);

        let rolls: Vec<_> = (1..=3)
            .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
            .collect();

        for index in (0..self.players.len()).cycle() {
            println!("PLAYER: {}", index);
            let mut next: HashMap<Game, u128> = HashMap::new();
            for &roll in rolls.iter() {
                println!("  Roll: {}", roll);
                for (game, universes) in games.iter() {
                    println!("    Game: {:?}, universes: {}", game, universes);
                    let player = self.players[index].borrow_mut();
                    player.roll(roll);

                    if player.score >= 21 {
                        wins[index] += universes;
                    } else {
                        *next.entry(game.clone()).or_insert(0) += universes;
                    }
                }
            }

            games = next;
            if games.is_empty() {
                break;
            }
        }

        *wins.iter().max().unwrap()
    }
}

fn main() {
    // Given input, player1 starts at 7, player2 starts at 3.
    let mut game = Game::new(7, 3);
    let dice = DeterministicDice::default();
    let score = game.play1(dice);
    dbg!(score);

    let mut game = Game::new(7, 3);
    let score = game.play2();
    dbg!(score);
}

#[cfg(test)]
mod tests {
    use crate::{DeterministicDice, Game};

    #[test]
    fn test_deterministic_game() {
        let mut game = Game::new(4, 8);
        let dice = DeterministicDice::default();
        assert_eq!(739785, game.play1(dice));
    }

    #[test]
    fn test_multiverse_game() {
        let mut game = Game::new(4, 8);
        assert_eq!(444356092776315, game.play2());
    }
}
