use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => unreachable!(),
        }
    }
}

impl Amphipod {
    pub fn energy(&self) -> i32 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FieldState {
    /// The corridor
    Corridor,
    /// Entrance right in front of the room
    Entrance,
    /// The room with designated amphipod
    Room(Amphipod),
    /// A wall
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    pub x: i32,
    pub y: i32,
    pub state: FieldState,
}

impl Field {
    pub fn new(x: i32, y: i32, state: FieldState) -> Self {
        Self { x, y, state }
    }

    pub fn is_corridor(&self) -> bool {
        self.state == FieldState::Corridor
    }

    pub fn is_room(&self) -> bool {
        match self.state {
            FieldState::Room(_) => true,
            _ => false,
        }
    }

    pub fn is_home(&self, amphipod: &Amphipod) -> bool {
        match self.state {
            FieldState::Room(ref a) => a == amphipod,
            _ => false,
        }
    }

    pub fn distance(&self, dest: &Field) -> i32 {
        (dest.x - self.x).abs() + (dest.y - self.y).abs()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match &self.state {
            FieldState::Corridor => ".".to_string(),
            FieldState::Entrance => "+".to_string(),
            FieldState::Room(_) => "_".to_string(),
            FieldState::Wall => "#".to_string(),
        };
        write!(f, "{}", x)
    }
}

/// Keeps information on all amphipods and their cost
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pub amphipods: Vec<(Field, Amphipod)>,
    pub cost: usize,
}

impl State {
    pub fn new(amphipods: Vec<(Field, Amphipod)>) -> Self {
        Self { amphipods, cost: 0 }
    }

    /// Returns true if all amphipods are in their designated rooms
    pub fn is_finished(&self) -> bool {
        self.amphipods.iter().all(|(field, amphipod)| {
            field.state == FieldState::Room(*amphipod)
        })
    }

    pub fn is_homeroom(&self) -> bool {
        false
    }
}

impl Ord for State {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<Field>,
    pub state: State,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn new(fields: Vec<Field>, amphipods: Vec<(Field, Amphipod)>) -> Self {
        let width = fields.iter().map(|f| f.x).max().unwrap() as u32 + 1;
        let height = fields.iter().map(|f| f.y).max().unwrap() as u32 + 1;

        Self {
            fields,
            state: State::new(amphipods),
            width,
            height,
        }
    }

    /// Get the next moves for all amphipods
    /// Returns index of amphipod and their next field.
    fn get_next_moves(&self, state: &State) -> Vec<(usize, &Field)> {
        let mut result = Vec::new();

        for (index, (field, amphipod)) in state.amphipods.iter().enumerate() {
            if field.is_corridor() {
                // TODO?

                for dst in self.home_positions(*amphipod) {

                }
            }
        }

        result
    }

    /// Moves all amphiods into their rooms, calculates minimum possible total entry
    pub fn organize(&self) -> Option<usize> {
        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        queue.push(self.state.clone());

        let mut costs: HashMap<State, usize> = HashMap::new();
        costs.insert(self.state.clone(), 0);

        while let Some(state) = queue.pop() {
            if state.is_finished() {
                return Some(state.cost);
            }

            if let Some(lowest_cost) = costs.get(&state) {
                if *lowest_cost < state.cost {
                    continue;
                }
            }

            for (index, next_pos) in self.get_next_moves(&state) {
                let (current_pos, amphipod) = &state.amphipods[index];

                let mut next_state = state.clone();
                next_state.amphipods[index] = (next_pos.clone(), *amphipod);

                let cost = state.cost + (current_pos.distance(&next_pos) * amphipod.energy()) as usize;
                if let Some(lowest_cost) = costs.get(&next_state) {
                    if *lowest_cost <= cost {
                        continue;
                    }
                }
                costs.insert(next_state.clone(), cost);
                queue.push(next_state.clone());
            }
        }

        None
    }

    /// Returns all home positions for given amphipod
    pub fn home_positions(&self, amphipod: Amphipod) -> Vec<(i32, i32)> {
        self.fields.iter().filter(|&f| f.is_home(&amphipod)).map(|f| (f.x, f.y)).collect_vec()
    }

    fn neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = Option<&Field>> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |&(nx, ny)| self.get(x as i32 + nx, y as i32 + ny))
    }

    fn get(&self, x: i32, y: i32) -> Option<&Field> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields.get((y as u32 * self.width + x as u32) as usize)
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(field) = self.get(x as i32, y as i32) {
                    write!(f, "{}", field)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Parses the input
fn parse_input(input: &str) -> Grid {
    let entrances = [(3, 1), (5, 1), (7, 1), (9, 1)]
        .into_iter()
        .collect::<HashSet<_>>();
    let designated_rooms = [
        (3, Amphipod::Amber),
        (5, Amphipod::Bronze),
        (7, Amphipod::Copper),
        (9, Amphipod::Desert),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut fields = Vec::new();
    let mut amphipods = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match c {
                '#' | ' ' => fields.push(Field::new(x, y, FieldState::Wall)),
                '.' => {
                    if entrances.contains(&(x, y)) {
                        fields.push(Field::new(x, y, FieldState::Entrance));
                    } else {
                        fields.push(Field::new(x, y, FieldState::Corridor));
                    }
                }
                'A' | 'B' | 'C' | 'D' => {
                    let designated = &designated_rooms[&(x as usize)];
                    let field = Field::new(x, y, FieldState::Room(designated.clone()));
                    fields.push(field.clone());
                    amphipods.push((field, Amphipod::from(c)));
                }
                _ => unreachable!(),
            }
        }
    }

    Grid::new(fields, amphipods)
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    dbg!(grid.organize());
}

#[cfg(test)]
mod tests {
    use crate::{Amphipod, Field, FieldState, State, parse_input};

    const INPUT: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########  "#;

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(8, grid.state.amphipods.len());
    }

    #[test]
    fn test_state_is_finished() {
        let amphipod = Amphipod::Amber;
        let state = State::new(vec![(Field::new(1, 1, FieldState::Room(amphipod)), amphipod)]);
        assert!(state.is_finished());
    }

    #[test]
    fn test_state_is_not_finished() {
        let fields = vec![
            Field::new(2, 2, FieldState::Corridor),
            Field::new(1, 2, FieldState::Room(Amphipod::Bronze)),
        ];
        let amphipods = vec![
            (fields[0].clone(), Amphipod::Desert),
            (fields[1].clone(), Amphipod::Amber),
        ];
        let state = State::new(amphipods);
        assert!(!state.is_finished());
    }

    #[test]
    fn test_get_home_positions() {
        let grid = parse_input(INPUT);
        assert_eq!(vec![(3, 2), (3, 3)], grid.home_positions(Amphipod::Amber));
        assert_eq!(vec![(5, 2), (5, 3)], grid.home_positions(Amphipod::Bronze));
        assert_eq!(vec![(7, 2), (7, 3)], grid.home_positions(Amphipod::Copper));
        assert_eq!(vec![(9, 2), (9, 3)], grid.home_positions(Amphipod::Desert));
    }

    #[test]
    fn test_organize_amphipods() {
        let grid = parse_input(INPUT);
        assert_eq!(12521, grid.organize().unwrap());
    }
}
