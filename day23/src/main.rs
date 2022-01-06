use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum FieldType {
    /// The corridor
    Corridor,
    /// Entrance right in front of the room
    Entrance,
    /// The room with designated amphipod
    Room(Type),
    /// A wall
    Wall,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Field {
    pub pos: Pos,
    pub state: FieldType,
}

impl Field {
    pub fn new(x: i32, y: i32, state: FieldType) -> Self {
        Self {
            pos: Pos::new(x, y),
            state,
        }
    }

    #[inline(always)]
    pub fn is_wall(&self) -> bool {
        self.state == FieldType::Wall
    }

    #[inline(always)]
    pub fn is_hallway(&self) -> bool {
        self.state == FieldType::Corridor
    }

    #[inline(always)]
    pub fn is_home(&self, amphipod: Type) -> bool {
        self.state == FieldType::Room(amphipod)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Type {
    fn energy(&self) -> u32 {
        match self {
            Type::Amber => 1,
            Type::Bronze => 10,
            Type::Copper => 100,
            Type::Desert => 1000,
        }
    }
}

impl From<char> for Type {
    fn from(c: char) -> Self {
        match c {
            'A' => Type::Amber,
            'B' => Type::Bronze,
            'C' => Type::Copper,
            'D' => Type::Desert,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Amphipod {
    typ: Type,
    pos: Pos,
}

impl Amphipod {
    pub fn new(pos: Pos, amphipod: Type) -> Self {
        Self { pos, typ: amphipod }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pub amphipods: Vec<Amphipod>,
}

impl State {
    pub fn new(amphipods: Vec<Amphipod>) -> Self {
        Self { amphipods }
    }

    pub fn get(&self, rhs: &Pos) -> Option<&Amphipod> {
        self.amphipods.iter().find(|&amphipod| amphipod.pos == *rhs)
    }

    pub fn blocks(&self, rhs: &Pos) -> bool {
        self.get(rhs).is_some()
    }
}

/// Represents the full 2d grid parsed from input.
#[derive(Debug)]
struct Grid {
    pub fields: Vec<Field>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn new(fields: Vec<Field>) -> Self {
        let width = fields.iter().map(|f| f.pos.x).max().unwrap() as u32 + 1;
        let height = fields.iter().map(|f| f.pos.y).max().unwrap() as u32 + 1;

        Self {
            fields,
            width,
            height,
        }
    }

    pub fn organize(&self, state: &State, cost: u32, cache: &mut HashMap<State, u32>) -> u32 {
        if self.is_finished(state) {
            return cost;
        }

        let mut best = u32::MAX;

        for (index, amphipod) in state.amphipods.iter().enumerate() {
            let paths = self.find_next_moves(amphipod, state);

            for path in paths {
                if let Some(Field { pos: next_pos, .. }) = path.last() {
                    let amphipod = state.amphipods[index];
                    let mut next_state = state.clone();

                    next_state.amphipods[index] = Amphipod::new(*next_pos, amphipod.typ);
                    next_state.amphipods.sort();

                    let next_cost = cost + (path.len() - 1) as u32 * amphipod.typ.energy();
                    if let Some(&prev_cost) = cache.get(&next_state) {
                        if prev_cost <= next_cost {
                            continue;
                        }
                    }

                    cache.insert(next_state.clone(), next_cost);
                    best = best.min(self.organize(&next_state, next_cost, cache));
                }
            }
        }

        best
    }

    fn is_finished(&self, state: &State) -> bool {
        state.amphipods.iter().all(|amphipod| {
            if let Some(room) = self.get(amphipod.pos.x, amphipod.pos.y) {
                room.is_home(amphipod.typ)
            } else {
                false
            }
        })
    }

    fn find_next_moves(&self, amphipod: &Amphipod, state: &State) -> Vec<Vec<&Field>> {
        let mut result = Vec::new();

        let &Amphipod { pos, .. } = amphipod;

        let field = self.get(pos.x, pos.y).expect("Failed to get field");

        // get designated home room
        let intended_pos = self.get_home_room(&amphipod, &state);
        if intended_pos == pos {
            return result;
        }

        if let Some(path) = self.find_path(pos, intended_pos, &state) {
            result.push(path);
            return result;
        }

        // Amphipod is already on one of the hallway fields, keep it there until it can move to a room.
        if field.is_hallway() {
            return result;
        }

        // Amphipod is in the wrong room, there is no unhindered path to the correct room.
        // Therefore can only move to a field in the hallway.
        for dest in self.free_hallway_spaces(&state) {
            if let Some(path) = self.find_path(pos, dest.pos, &state) {
                result.push(path);
            }
        }

        result
    }

    /// Return the desired home room field of the given amphipod
    fn get_home_room(&self, amphipod: &Amphipod, state: &State) -> Pos {
        for dest in self.home_rooms(amphipod.typ) {
            if dest.pos == amphipod.pos {
                return dest.pos;
            }

            match state.get(&dest.pos) {
                None => return dest.pos,
                Some(&other) => {
                    if other.typ != amphipod.typ {
                        return dest.pos;
                    }
                }
            }
        }

        unreachable!()
    }

    /// A simple breadth first search algorithm that returns the shortest path to the destination
    /// or `None` if there is none present.
    pub fn find_path(&self, source: Pos, dest: Pos, state: &State) -> Option<Vec<&Field>> {
        if source == dest {
            return None;
        }

        let mut paths: VecDeque<Vec<&Field>> = VecDeque::new();
        let mut visited: Vec<&Field> = Vec::new();
        paths.push_back(vec![self.get_field(source.x, source.y)]);

        while let Some(path) = paths.pop_front() {
            let last = *path.last().expect("No last element found");
            visited.push(last);
            if last.pos == dest {
                return Some(path);
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (last.pos.x + dx, last.pos.y + dy);
                if let Some(neighbor) = self.get(x, y) {
                    if neighbor.is_wall() {
                        continue;
                    }

                    if visited.contains(&neighbor) {
                        continue;
                    }

                    if state.blocks(&neighbor.pos) {
                        continue;
                    }

                    let mut path = path.clone();
                    path.push(neighbor);
                    paths.push_back(path);
                }
            }
        }

        None
    }

    fn home_rooms(&self, amphipod: Type) -> Vec<&Field> {
        self.fields
            .iter()
            .rev()
            .filter(|&f| f.is_home(amphipod))
            .collect_vec()
    }

    fn free_hallway_spaces(&self, state: &State) -> Vec<&Field> {
        self.fields
            .iter()
            .filter(|&f| f.is_hallway() && !state.blocks(&f.pos))
            .collect_vec()
    }

    fn get_field(&self, x: i32, y: i32) -> &Field {
        assert!(0 <= x && x < self.width as i32);
        assert!(0 <= y && y < self.height as i32);
        &self.fields[(y * self.width as i32 + x) as usize]
    }

    fn get(&self, x: i32, y: i32) -> Option<&Field> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields.get((y as u32 * self.width + x as u32) as usize)
        } else {
            None
        }
    }
}

/// Parses the input
fn parse_input(input: &str) -> (Grid, State) {
    let entrances = [(3, 1), (5, 1), (7, 1), (9, 1)]
        .into_iter()
        .collect::<HashSet<_>>();
    let designated_rooms = [
        (3, Type::Amber),
        (5, Type::Bronze),
        (7, Type::Copper),
        (9, Type::Desert),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut fields = Vec::new();
    let mut amphipods = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match c {
                '#' | ' ' => fields.push(Field::new(x, y, FieldType::Wall)),
                '.' => {
                    if entrances.contains(&(x, y)) {
                        fields.push(Field::new(x, y, FieldType::Entrance));
                    } else {
                        fields.push(Field::new(x, y, FieldType::Corridor));
                    }
                }
                'A' | 'B' | 'C' | 'D' => {
                    let designated = &designated_rooms[&(x as usize)];
                    let field = Field::new(x, y, FieldType::Room(*designated));
                    fields.push(field.clone());
                    amphipods.push(Amphipod::new(field.pos, Type::from(c)))
                }
                _ => unreachable!(),
            }
        }
    }
    amphipods.sort();

    (Grid::new(fields), State::new(amphipods))
}

fn main() {
    let (grid, start) = parse_input(include_str!("input1.txt"));
    let cost = grid.organize(&start, 0, &mut HashMap::new());
    assert_eq!(11320, cost);
    dbg!(cost);

    let (grid, start) = parse_input(include_str!("input2.txt"));
    let cost = grid.organize(&start, 0, &mut HashMap::new());
    assert_eq!(49532, cost);
    dbg!(cost);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_input, Pos};

    const INPUT: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########  "#;

    const INPUT2: &str = r#"#############
#...........#
###B#C#B#D###
  #D#C#B#A#  
  #D#B#A#C#  
  #A#D#C#A#  
  #########  "#;

    #[test]
    fn test_multiple_paths() {
        let (grid, mut start) = parse_input(INPUT2);

        assert!(grid
            .find_path(Pos::new(3, 2), Pos::new(1, 1), &mut start)
            .is_some());
        assert!(grid
            .find_path(Pos::new(3, 2), Pos::new(2, 1), &mut start)
            .is_some());
        assert!(grid
            .find_path(Pos::new(5, 2), Pos::new(3, 1), &mut start)
            .is_some());
        assert!(grid
            .find_path(Pos::new(5, 2), Pos::new(4, 1), &mut start)
            .is_some());
        assert!(grid
            .find_path(Pos::new(9, 2), Pos::new(9, 1), &mut start)
            .is_some());

        assert!(grid
            .find_path(Pos::new(3, 3), Pos::new(3, 1), &mut start)
            .is_none());
        assert!(grid
            .find_path(Pos::new(5, 3), Pos::new(4, 1), &mut start)
            .is_none());
        assert!(grid
            .find_path(Pos::new(7, 4), Pos::new(5, 1), &mut start)
            .is_none());
        assert!(grid
            .find_path(Pos::new(9, 5), Pos::new(6, 1), &mut start)
            .is_none());
    }

    #[test]
    fn test_organize_amphipods() {
        let (grid, start) = parse_input(INPUT);
        assert_eq!(12521, grid.organize(&start, 0, &mut HashMap::new()));
    }

    #[test]
    fn test_organize_2nd_solution() {
        let (grid, start) = parse_input(INPUT2);
        assert_eq!(44169, grid.organize(&start, 0, &mut HashMap::new()));
    }
}
