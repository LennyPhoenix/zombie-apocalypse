use std::{
    cmp::{max, min},
    fmt::Display,
};

pub enum DeathCheckResult {
    Alive,
    Dead,
    Undead,
}

pub const NAME_POOL: &[&str] = &[
    "Fred",
    "Shaggy",
    "Daphne",
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
];

#[derive(PartialEq, Clone)]
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
        Self::new("Velma", 20, 2, None)
    }

    pub fn scoob() -> Self {
        Self::new("Scoob", 15, 1, None)
    }

    pub fn check_dead(&self) -> DeathCheckResult {
        if self.hp <= 0 {
            if self.infection_level > 20 {
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
}
