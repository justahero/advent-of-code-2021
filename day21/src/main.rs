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

    pub fn play2(&mut self) -> usize {
        let mut cache = HashMap::new();
        let result = self.play_recursive(&mut cache, 0);
        println!("RESULT: {:?}", result);
        *result.iter().max().unwrap()
    }

    fn play_recursive(&self, cache: &mut HashMap<Game, [usize; 2]>, index: usize) -> [usize; 2] {
        if cache.contains_key(&self) {
            return *cache.get(&self).unwrap();
        }

        let mut result = [0, 0];

        let rolls: Vec<_> = (1_u32..=3)
            .flat_map(|a| (1_u32..=3).flat_map(move |b| (1_u32..=3).map(move |c| a + b + c)))
            .collect();

        for roll in rolls.iter() {
            let mut next_game = self.clone();

            let player = next_game.players[index].borrow_mut();
            player.roll(*roll);

            if player.score >= 21 {
                result[index] += 1;
            } else {
                let r = next_game.play_recursive(cache, 1 - index);
                result[0] += r[0];
                result[1] += r[1];
            }
        }

        cache.insert(self.clone(), result);
        result
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
    // 137221844288868 too low
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
