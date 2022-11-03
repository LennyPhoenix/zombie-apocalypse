use std::fmt::Display;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::location::Location;

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub seen: bool,
    pub explored: bool,
    pub location_type: Option<Location>,
}

impl Tile {
    pub fn random() -> Self {
        let loc_type;

        if thread_rng().gen_bool(0.15) {
            loc_type = Some(Location::random());
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
            if let None = &self.location_type {
                if self.explored {
                    char = '#';
                } else {
                    char = '.';
                }
            } else if self.explored {
                char = 'X';
            } else {
                char = '?';
            }
        } else {
            char = ' ';
        }
        write!(f, "{}", char)
    }
}
