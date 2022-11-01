use std::fmt::Display;
use std::fmt::Write;

use rand::{seq::IteratorRandom, thread_rng, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Map {
    position: Position,
    width: i32,
    height: i32,
    rows: Vec<Vec<Tile>>,
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = thread_rng();

        let mut rows = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                let tile = Tile::random();
                row.push(tile);
            }
            rows.push(row);
        }

        let position = Position {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        };

        let mut map = Self {
            position,
            rows,
            width,
            height,
        };
        map.check_surroundings();

        map
    }

    pub fn get_pos(&self) -> &Position {
        &self.position
    }

    pub fn explore(&mut self) {
        self.get_tile_mut(None).explored = true;
    }

    pub fn get_tile(&self, pos: Option<&Position>) -> &Tile {
        let position;
        if let Some(pos) = pos {
            position = pos;
        } else {
            position = &self.position;
        }

        &self.rows[position.y as usize][position.x as usize]
    }

    fn get_tile_mut(&mut self, pos: Option<&Position>) -> &mut Tile {
        let position;
        if let Some(pos) = pos {
            position = pos;
        } else {
            position = &self.position;
        }

        &mut self.rows[position.y as usize][position.x as usize]
    }

    fn check_surroundings(&mut self) {
        for x in -1..=1 {
            for y in -1..=1 {
                let pos = Position {
                    x: (self.position.x + x).rem_euclid(self.width),
                    y: (self.position.y + y).rem_euclid(self.height),
                };

                self.get_tile_mut(Some(&pos)).seen = true;
            }
        }
    }

    pub fn travel(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.position.y -= 1,
            Direction::South => self.position.y += 1,
            Direction::East => self.position.x += 1,
            Direction::West => self.position.x -= 1,
        };

        self.position = Position {
            x: self.position.x.rem_euclid(self.width),
            y: self.position.y.rem_euclid(self.height),
        };

        self.check_surroundings();
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const KEY: &[&str] = &[
            ".  Unexplored",
            "?  Point of Interest",
            "#  Explored",
            "X  Explored Point of Interest",
            "M  Mystery Machine",
        ];

        let mut output = String::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x as i32 != self.position.x || y as i32 != self.position.y {
                    write!(output, "{tile}")?;
                } else {
                    write!(output, "M")?;
                }
            }
            if y < KEY.len() {
                write!(output, " | {}", KEY[y])?;
            }
            write!(output, "\n")?;
        }

        write!(f, "{}", output)
    }
}

#[derive(EnumIter)]
pub enum Location {
    ShoppingCentre,
    TradeWell,
    MilitaryBase,
    Desert,
}

pub struct Tile {
    explored: bool,
    seen: bool,
    location_type: Option<Location>,
}

impl Tile {
    fn random() -> Self {
        let mut rng = thread_rng();

        let loc_type;

        if rng.gen_bool(0.15) {
            loc_type = Location::iter().choose(&mut rng);
        } else {
            loc_type = None;
        }

        Self {
            explored: false,
            seen: false,
            location_type: loc_type,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char;
        if self.seen {
            if let Some(_) = &self.location_type {
                if self.explored {
                    char = 'X';
                } else {
                    char = '?';
                }
            } else {
                if self.explored {
                    char = '#';
                } else {
                    char = '.';
                }
            }
        } else {
            char = ' ';
        }
        write!(f, "{}", char)
    }
}
