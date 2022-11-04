use std::fmt::Display;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{event::Event, location::Location, time::Time};

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub seen: bool,
    pub explored: bool,
    pub location_type: Option<Location>,
}

impl Tile {
    pub fn random() -> Self {
        let loc_type = if thread_rng().gen_bool(0.15) {
            Some(Location::random())
        } else {
            None
        };

        Self {
            explored: false,
            seen: false,
            location_type: loc_type,
        }
    }

    pub fn get_event_options(&self, time: &Time) -> &[Event] {
        if let Some(location) = &self.location_type {
            if !time.night() {
                match location {
                    Location::ShoppingCentre => &[
                        Event::Food(2),
                        Event::Food(2),
                        Event::Zombie(4),
                        Event::Zombie(4),
                        Event::Fuel(1),
                        Event::Money(6),
                        Event::Survivor(1),
                    ],
                    Location::TradeWell => &[
                        Event::Zombie(5),
                        Event::Zombie(5),
                        Event::Money(3),
                        Event::Ammo(4),
                        Event::Fuel(1),
                        Event::Survivor(1),
                    ],
                    Location::MilitaryBase => &[
                        Event::Zombie(5),
                        Event::Zombie(7),
                        Event::Ammo(5),
                        Event::Fuel(2),
                        Event::Money(4),
                        Event::Survivor(2),
                    ],
                }
            } else {
                match location {
                    Location::ShoppingCentre => &[
                        Event::Nothing,
                        Event::Nothing,
                        Event::Zombie(6),
                        Event::Zombie(6),
                        Event::Fuel(1),
                    ],
                    Location::TradeWell => &[
                        Event::Nothing,
                        Event::Nothing,
                        Event::Zombie(5),
                        Event::Zombie(5),
                        Event::Ammo(4),
                        Event::Fuel(1),
                    ],
                    Location::MilitaryBase => &[
                        Event::Nothing,
                        Event::Nothing,
                        Event::Zombie(5),
                        Event::Zombie(7),
                        Event::Ammo(5),
                        Event::Fuel(2),
                    ],
                }
            }
        } else if !time.night() {
            &[
                Event::Zombie(4),
                Event::Zombie(4),
                Event::Zombie(4),
                Event::Food(3),
                Event::Food(3),
                Event::Money(3),
                Event::Money(3),
                Event::Ammo(2),
                Event::Fuel(2),
                Event::Survivor(2),
            ]
        } else {
            &[
                Event::Zombie(5),
                Event::Zombie(5),
                Event::Nothing,
                Event::Fuel(1),
            ]
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char;
        if self.seen {
            if self.location_type.is_none() {
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
