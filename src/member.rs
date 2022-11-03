use std::{
    cmp::{max, min},
    fmt::Display,
};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::io::ellipsis;

const REANIMATION_THRESHOLD: i32 = 20;
const INFECTION_DAMAGE_THRESHOLD: i32 = 25;
const INFECTION_DAMAGE: i32 = 4;

pub enum DeathCheckResult {
    Alive,
    Dead,
    Undead,
}

#[derive(Serialize, Deserialize)]
pub struct NamePool {
    available: Vec<String>,
}

impl NamePool {
    pub fn new() -> Self {
        Self {
            available: vec![
                "Scrappy-Doo",
                "Scooby-Dum",
                "Yabba-Doo",
                "Scooby-Dee",
                "Vincent Van Ghoul",
                "Film Flam",
                "Weerd",
                "Bogel",
                "Thorn",
                "Dusk",
                "Luna",
                "Maldor",
                "Morbida",
                "Spectre",
                "Zomba",
                "Captain Ferguson",
                "Nekara",
                "Marcella",
                "Demondo",
                "Rankor",
                "Phantazmo",
                "Zimbulu",
                "Asmodeus",
                "Freako",
                "Meako",
                "Shreako",
                "Sadie-Mae Scroggins",
                "Billy-Bob Scroggins",
            ]
            .iter()
            .map(|string: &&str| string.to_string())
            .collect(),
        }
    }

    pub fn get(&mut self) -> String {
        self.available.shuffle(&mut thread_rng());
        return match self.available.pop() {
            Some(name) => name,
            None => NamePool::new().get(),
        };
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub infection_level: i32,
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\n- HP: {}/{}\n- Infection: {}",
            self.name, self.hp, self.max_hp, self.infection_level
        )
    }
}

impl Member {
    pub fn new(name: &str, max_hp: i32, infection_level: i32, current_hp: Option<i32>) -> Self {
        Self {
            name: name.to_owned(),
            hp: match current_hp {
                Some(hp) => hp,
                None => max_hp,
            },
            max_hp,
            infection_level,
        }
    }

    pub fn velma() -> Self {
        Self::new("Velma", 17, 3, None)
    }

    pub fn shaggy() -> Self {
        Self::new("Shaggy", 12, 0, None)
    }

    pub fn fred() -> Self {
        Self::new("Fred", 20, 7, None)
    }

    pub fn daphne() -> Self {
        Self::new("Daphne", 15, 1, None)
    }

    pub fn scoob() -> Self {
        Self::new("Scoob", 15, 0, None)
    }

    pub fn check_dead(&self) -> DeathCheckResult {
        if self.hp <= 0 {
            if self.infection_level >= REANIMATION_THRESHOLD {
                DeathCheckResult::Undead
            } else {
                DeathCheckResult::Dead
            }
        } else {
            DeathCheckResult::Alive
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp += amount;
        self.hp = min(self.hp, self.max_hp);
    }

    pub fn cure(&mut self, amount: i32) {
        self.infection_level -= amount;
        self.infection_level = max(self.infection_level, 0);
    }

    pub fn hurt(&mut self, damage: i32) {
        if damage >= 6 {
            println!(
                "{} takes {} damage, and becomes much more infected...",
                self.name, damage
            );
            self.infection_level += 5;
        } else {
            println!(
                "{} takes {} damage, and becomes slightly more infected...",
                self.name, damage
            );
            self.infection_level += 2;
        }

        self.hp -= min(self.hp, damage);
    }

    pub fn check_infection(&mut self) -> DeathCheckResult {
        if self.infection_level >= INFECTION_DAMAGE_THRESHOLD {
            self.hp -= INFECTION_DAMAGE;
            print!(
                "{} coughs violently and takes {INFECTION_DAMAGE} damage",
                self.name
            );
            ellipsis();
            println!();
        }
        self.check_dead()
    }
}
