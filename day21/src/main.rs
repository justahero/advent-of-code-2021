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
        let values = (1..=times).map(|step| ((self.start + step - 1) % 100) + 1).collect_vec();
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
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(player1: u8, player2: u8) -> Self {
        Self {
            players: vec![Player::new(player1), Player::new(player2)],
        }
    }

    /// Plays until one player wins
    pub fn play1(&mut self, mut dice: impl Roll) -> u32 {
        let mut roll_count = 0_u32;

        for index in (0..self.players.len()).cycle() {
            let values = dice.roll(3);
            roll_count += 3;

            if self.players[index].play(&values) >= 1000 {
                break;
            }
        }

        // get losing player
        let score = self.players.iter().map(|p| p.score).min().unwrap();
        score * roll_count
    }
}

fn main() {
    // Given input, player1 starts at 7, player2 starts at 3.
    let mut game = Game::new(7, 3);
    let dice = DeterministicDice::default();
    let score = game.play1(dice);
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
}
