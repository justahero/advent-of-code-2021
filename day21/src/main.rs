use itertools::Itertools;

trait Roll {
    fn roll(&self) -> Vec<u32>;
}

struct DeterministicDice {
    pub start: u32,
}

impl Default for DeterministicDice {
    fn default() -> Self {
        Self { start: 1 }
    }
}

impl Roll for DeterministicDice {
    fn roll(&self) -> Vec<u32> {
        let values = [
            (self.start + 1) % 100,
            (self.start + 2) % 100,
            (self.start + 3) % 100,
        ];
        values.iter().cloned().collect_vec()
    }
}

#[derive(Debug)]
struct Player {
    pub start: u8,
    pub score: u32,
}

impl Player {
    pub fn new(start: u8) -> Self {
        Self { start, score: 0 }
    }
}

#[derive(Debug)]
struct Game {
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player1: u8, player2: u8) -> Self {
        Self {
            player1: Player::new(player1),
            player2: Player::new(player2),
        }
    }

    pub fn play(&mut self, dice: impl Roll) -> &Player {
        // play until player arrives
        &self.player1
    }
}

fn main() {
    let mut game = Game::new(1, 1);
    let dice = DeterministicDice::default();
    game.play(dice);
}

#[cfg(test)]
mod tests {
    use crate::{DeterministicDice, Game};

    #[test]
    fn test_deterministic_game() {
        let mut game = Game::new(1, 1);
        let dice = DeterministicDice::default();
        let winner = game.play(dice);
    }
}
