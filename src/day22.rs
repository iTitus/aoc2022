use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::Vector2;
use num::integer::gcd;
use rustc_hash::FxHashMap;

pub type Pos = Vector2<i32>;

const NORTH: Pos = Pos::new(0, -1);
const EAST: Pos = Pos::new(1, 0);
const SOUTH: Pos = Pos::new(0, 1);
const WEST: Pos = Pos::new(-1, 0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn vec(&self) -> &'static Pos {
        match self {
            Dir::North => &NORTH,
            Dir::East => &EAST,
            Dir::South => &SOUTH,
            Dir::West => &WEST,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Face {
    tiles: Vec<Tile>,
    size: usize,
}

impl Face {
    pub fn first_free(&self) -> Pos {
        let i = self.tiles.iter().position(|t| *t == Tile::Empty).unwrap();
        Pos::new((i % self.size) as i32, (i / self.size) as i32)
    }
    pub fn can_go(&self, pos: &Pos) -> bool {
        let i = pos.x as usize + self.size * pos.y as usize;
        self.tiles[i] == Tile::Empty
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    Left,
    Right,
    Forward(usize),
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (usize, usize, FxHashMap<Pos, Face>, Vec<Command>) {
    fn min_face_len(it: impl IntoIterator<Item = char>) -> usize {
        it.into_iter()
            .chunk_by(|&c| c == ' ')
            .into_iter()
            .filter_map(|(_, g)| {
                let c = g.count();
                if c > 0 {
                    Some(c)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    let (map, commands) = input.split_once("\n\n").unwrap();
    let map = map.lines().collect_vec();
    let min_face_len_horizontal = map.iter().map(|&l| min_face_len(l.chars())).min().unwrap();
    let min_face_len_vertical = (0..(map.iter().map(|l| l.len()).max().unwrap()))
        .map(|i| {
            map.iter().map(move |&l| {
                let bytes = l.as_bytes();
                if i >= bytes.len() {
                    ' '
                } else {
                    bytes[i] as char
                }
            })
        })
        .map(min_face_len)
        .min()
        .unwrap();
    let size = gcd(min_face_len_horizontal, min_face_len_vertical);

    let max_width = map.iter().map(|l| l.len()).max().unwrap();
    assert_eq!(0, max_width % size);
    let max_width = max_width / size;

    let mut faces: FxHashMap<Pos, Face> = FxHashMap::default();
    let first_face_offset = map[0].bytes().position(|b| b != b' ').unwrap();
    assert_eq!(0, first_face_offset % size);
    let first_face_offset = first_face_offset / size;

    for (y, y_block) in map.chunks(size).enumerate() {
        for x in 0..max_width {
            if y_block[0].len() <= x * size || y_block[0].as_bytes()[x * size] == b' ' {
                continue;
            }

            let tiles = y_block
                .iter()
                .flat_map(|l| l[(x * size)..((x + 1) * size)].bytes())
                .map(|b| match b {
                    b'.' => Tile::Empty,
                    b'#' => Tile::Wall,
                    _ => panic!(),
                })
                .collect_vec();

            let real_x = x as i32 - first_face_offset as i32;
            let real_y = y as i32;
            let pos = Pos::new(real_x, real_y);
            let face = Face { tiles, size };

            faces.insert(pos, face);
        }
    }

    let commands = commands.trim();
    let mut command_vec = vec![];
    let mut number_start = None;
    for (i, c) in commands.char_indices() {
        match c {
            'L' => {
                if let Some(start) = number_start {
                    command_vec.push(Command::Forward(commands[start..i].parse().unwrap()));
                    number_start = None;
                }
                command_vec.push(Command::Left);
            }
            'R' => {
                if let Some(start) = number_start {
                    command_vec.push(Command::Forward(commands[start..i].parse().unwrap()));
                    number_start = None;
                }
                command_vec.push(Command::Right);
            }
            _ => {
                if number_start.is_none() {
                    number_start = Some(i)
                }
            }
        }
    }

    if let Some(start) = number_start {
        command_vec.push(Command::Forward(commands[start..].parse().unwrap()));
    }

    (size, first_face_offset, faces, command_vec)
}

#[aoc(day22, part1)]
pub fn part1(
    (size, first_face_offset, faces, commands): &(usize, usize, FxHashMap<Pos, Face>, Vec<Command>),
) -> i32 {
    let mut face_pos = Pos::new(0, 0);
    let mut pos = faces[&face_pos].first_free();
    let mut dir = Dir::East;
    for &cmd in commands {
        match cmd {
            Command::Left => {
                dir = match dir {
                    Dir::North => Dir::West,
                    Dir::East => Dir::North,
                    Dir::South => Dir::East,
                    Dir::West => Dir::South,
                }
            }
            Command::Right => {
                dir = match dir {
                    Dir::North => Dir::East,
                    Dir::East => Dir::South,
                    Dir::South => Dir::West,
                    Dir::West => Dir::North,
                }
            }
            Command::Forward(distance) => {
                for _ in 0..distance {
                    let mut new_face_pos = face_pos;
                    let mut new_pos: Pos = pos + dir.vec();
                    if new_pos.x < 0
                        || new_pos.x >= *size as i32
                        || new_pos.y < 0
                        || new_pos.y >= *size as i32
                    {
                        new_pos.x = new_pos.x.rem_euclid(*size as i32);
                        new_pos.y = new_pos.y.rem_euclid(*size as i32);

                        new_face_pos += dir.vec();
                        if !faces.contains_key(&new_face_pos) {
                            new_face_pos = face_pos;
                            let opposite_dir = match dir {
                                Dir::North => Dir::South,
                                Dir::East => Dir::West,
                                Dir::South => Dir::North,
                                Dir::West => Dir::East,
                            }
                            .vec();
                            loop {
                                let new_new_face_pos: Pos = new_face_pos + opposite_dir;
                                if !faces.contains_key(&new_new_face_pos) {
                                    break;
                                }

                                new_face_pos = new_new_face_pos;
                            }
                        }
                    }

                    if faces[&new_face_pos].can_go(&new_pos) {
                        face_pos = new_face_pos;
                        pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let row = 1 + pos.y + face_pos.y * *size as i32;
    let col = 1 + pos.x + (face_pos.x + *first_face_offset as i32) * *size as i32;
    let dir_value = match dir {
        Dir::North => 3,
        Dir::East => 0,
        Dir::South => 1,
        Dir::West => 2,
    };
    1000 * row + 4 * col + dir_value
}

#[aoc(day22, part2)]
pub fn part2(
    (size, first_face_offset, faces, commands): &(usize, usize, FxHashMap<Pos, Face>, Vec<Command>),
) -> i32 {
    let mut face_pos = Pos::new(0, 0);
    let mut pos = faces[&face_pos].first_free();
    let mut dir = Dir::East;
    for &cmd in commands {
        match cmd {
            Command::Left => {
                dir = match dir {
                    Dir::North => Dir::West,
                    Dir::East => Dir::North,
                    Dir::South => Dir::East,
                    Dir::West => Dir::South,
                }
            }
            Command::Right => {
                dir = match dir {
                    Dir::North => Dir::East,
                    Dir::East => Dir::South,
                    Dir::South => Dir::West,
                    Dir::West => Dir::North,
                }
            }
            Command::Forward(distance) => {
                for _ in 0..distance {
                    let mut new_face_pos = face_pos;
                    let mut new_pos: Pos = pos + dir.vec();
                    let mut new_dir = dir;
                    if new_pos.x < 0
                        || new_pos.x >= *size as i32
                        || new_pos.y < 0
                        || new_pos.y >= *size as i32
                    {
                        new_pos.x = new_pos.x.rem_euclid(*size as i32);
                        new_pos.y = new_pos.y.rem_euclid(*size as i32);

                        new_face_pos += dir.vec();
                        if !faces.contains_key(&new_face_pos) {
                            match size {
                                4 => {
                                    match (face_pos.x, face_pos.y, dir) {
                                        // Top <-> North
                                        (0, 0, Dir::North) => {
                                            new_face_pos = Pos::new(-2, 1);
                                            new_pos = Pos::new(*size as i32 - 1 - pos.x, 0);
                                            new_dir = Dir::South;
                                        }
                                        (-2, 1, Dir::North) => {
                                            new_face_pos = Pos::new(0, 0);
                                            new_pos = Pos::new(*size as i32 - 1 - pos.x, 0);
                                            new_dir = Dir::South;
                                        }

                                        // Top <-> West
                                        (0, 0, Dir::West) => {
                                            new_face_pos = Pos::new(-1, 1);
                                            new_pos = Pos::new(pos.y, 0);
                                            new_dir = Dir::South;
                                        }
                                        (-1, 1, Dir::North) => {
                                            new_face_pos = Pos::new(0, 0);
                                            new_pos = Pos::new(0, pos.x);
                                            new_dir = Dir::East;
                                        }

                                        // Top <-> East
                                        (0, 0, Dir::East) => {
                                            new_face_pos = Pos::new(1, 2);
                                            new_pos = Pos::new(
                                                *size as i32 - 1,
                                                *size as i32 - 1 - pos.y,
                                            );
                                            new_dir = Dir::West;
                                        }
                                        (1, 2, Dir::East) => {
                                            new_face_pos = Pos::new(0, 0);
                                            new_pos = Pos::new(
                                                *size as i32 - 1,
                                                *size as i32 - 1 - pos.y,
                                            );
                                            new_dir = Dir::West;
                                        }

                                        // South <-> East
                                        (0, 1, Dir::East) => {
                                            new_face_pos = Pos::new(1, 2);
                                            new_pos = Pos::new(*size as i32 - 1 - pos.y, 0);
                                            new_dir = Dir::South;
                                        }
                                        (1, 2, Dir::North) => {
                                            new_face_pos = Pos::new(0, 1);
                                            new_pos = Pos::new(
                                                *size as i32 - 1,
                                                *size as i32 - 1 - pos.x,
                                            );
                                            new_dir = Dir::West;
                                        }

                                        // West <-> Down
                                        (-1, 1, Dir::South) => {
                                            new_face_pos = Pos::new(0, 2);
                                            new_pos = Pos::new(0, *size as i32 - 1 - pos.x);
                                            new_dir = Dir::East;
                                        }
                                        (0, 2, Dir::West) => {
                                            new_face_pos = Pos::new(-1, 1);
                                            new_pos = Pos::new(
                                                *size as i32 - 1 - pos.y,
                                                *size as i32 - 1,
                                            );
                                            new_dir = Dir::North;
                                        }

                                        // North <-> Down
                                        (-2, 1, Dir::South) => {
                                            new_face_pos = Pos::new(0, 2);
                                            new_pos = Pos::new(
                                                *size as i32 - 1 - pos.x,
                                                *size as i32 - 1,
                                            );
                                            new_dir = Dir::North;
                                        }
                                        (0, 2, Dir::South) => {
                                            new_face_pos = Pos::new(-2, 1);
                                            new_pos = Pos::new(
                                                *size as i32 - 1 - pos.x,
                                                *size as i32 - 1,
                                            );
                                            new_dir = Dir::North;
                                        }

                                        // North <-> East
                                        (-2, 1, Dir::West) => {
                                            new_face_pos = Pos::new(1, 2);
                                            new_pos = Pos::new(
                                                *size as i32 - 1 - pos.y,
                                                *size as i32 - 1,
                                            );
                                            new_dir = Dir::North;
                                        }
                                        (1, 2, Dir::South) => {
                                            new_face_pos = Pos::new(-2, 1);
                                            new_pos = Pos::new(0, *size as i32 - 1 - pos.x);
                                            new_dir = Dir::East;
                                        }

                                        _ => unreachable!(),
                                    }
                                }
                                50 => {
                                    match (face_pos.x, face_pos.y, dir) {
                                        // Top <-> North
                                        (0, 0, Dir::North) => {
                                            new_face_pos = Pos::new(-1, 3);
                                            new_pos = Pos::new(0, pos.x);
                                            new_dir = Dir::East;
                                        }
                                        (-1, 3, Dir::West) => {
                                            new_face_pos = Pos::new(0, 0);
                                            new_pos = Pos::new(pos.y, 0);
                                            new_dir = Dir::South;
                                        }

                                        // Top <-> West
                                        (0, 0, Dir::West) => {
                                            new_face_pos = Pos::new(-1, 2);
                                            new_pos = Pos::new(0, *size as i32 - 1 - pos.y);
                                            new_dir = Dir::East;
                                        }
                                        (-1, 2, Dir::West) => {
                                            new_face_pos = Pos::new(0, 0);
                                            new_pos = Pos::new(0, *size as i32 - 1 - pos.y);
                                            new_dir = Dir::East;
                                        }

                                        // East <-> North
                                        (1, 0, Dir::North) => {
                                            new_face_pos = Pos::new(-1, 3);
                                            new_pos = Pos::new(pos.x, *size as i32 - 1);
                                            new_dir = Dir::North;
                                        }
                                        (-1, 3, Dir::South) => {
                                            new_face_pos = Pos::new(1, 0);
                                            new_pos = Pos::new(pos.x, 0);
                                            new_dir = Dir::South;
                                        }

                                        // East <-> Down
                                        (1, 0, Dir::East) => {
                                            new_face_pos = Pos::new(0, 2);
                                            new_pos = Pos::new(
                                                *size as i32 - 1,
                                                *size as i32 - 1 - pos.y,
                                            );
                                            new_dir = Dir::West;
                                        }
                                        (0, 2, Dir::East) => {
                                            new_face_pos = Pos::new(1, 0);
                                            new_pos = Pos::new(
                                                *size as i32 - 1,
                                                *size as i32 - 1 - pos.y,
                                            );
                                            new_dir = Dir::West;
                                        }

                                        // East <-> South
                                        (1, 0, Dir::South) => {
                                            new_face_pos = Pos::new(0, 1);
                                            new_pos = Pos::new(*size as i32 - 1, pos.x);
                                            new_dir = Dir::West;
                                        }
                                        (0, 1, Dir::East) => {
                                            new_face_pos = Pos::new(1, 0);
                                            new_pos = Pos::new(pos.y, *size as i32 - 1);
                                            new_dir = Dir::North;
                                        }

                                        // South <-> West
                                        (0, 1, Dir::West) => {
                                            new_face_pos = Pos::new(-1, 2);
                                            new_pos = Pos::new(pos.y, 0);
                                            new_dir = Dir::South;
                                        }
                                        (-1, 2, Dir::North) => {
                                            new_face_pos = Pos::new(0, 1);
                                            new_pos = Pos::new(0, pos.x);
                                            new_dir = Dir::East;
                                        }

                                        // Down <-> North
                                        (0, 2, Dir::South) => {
                                            new_face_pos = Pos::new(-1, 3);
                                            new_pos = Pos::new(*size as i32 - 1, pos.x);
                                            new_dir = Dir::West;
                                        }
                                        (-1, 3, Dir::East) => {
                                            new_face_pos = Pos::new(0, 2);
                                            new_pos = Pos::new(pos.y, *size as i32 - 1);
                                            new_dir = Dir::North;
                                        }

                                        _ => unreachable!(),
                                    }
                                }
                                _ => unimplemented!(),
                            }
                        }
                    }

                    if faces[&new_face_pos].can_go(&new_pos) {
                        face_pos = new_face_pos;
                        pos = new_pos;
                        dir = new_dir;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let row = 1 + pos.y + face_pos.y * *size as i32;
    let col = 1 + pos.x + (face_pos.x + *first_face_offset as i32) * *size as i32;
    let dir_value = match dir {
        Dir::North => 3,
        Dir::East => 0,
        Dir::South => 1,
        Dir::West => 2,
    };
    1000 * row + 4 * col + dir_value
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(
            r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
",
        );
        assert_eq!(6032, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(
            r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
",
        );
        assert_eq!(5031, part2(&input))
    }
}
