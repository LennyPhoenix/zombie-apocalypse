use std::fs::{read_to_string, remove_file, write};

use serde::{Deserialize, Serialize};

use crate::{
    io::{clear, ellipsis, flush, pause, read_line, wait},
    map::Map,
    member::NamePool,
    party::Party,
    time::Time,
};

const SAVE_LOCATION: &str = "savegame.json";

#[derive(Serialize, Deserialize)]
pub struct Game {
    party: Party,
    map: Map,
    time: Time,
    name_pool: NamePool,
}

impl Game {
    fn new() -> Self {
        print!("Day 0");
        ellipsis();
        println!("\nEver since the outbreak, the gang were completely separated.");
        pause();
        println!("With the power grid down, its almost impossible to contact others.");
        pause();
        println!("Resources are limited, and the infection gets worse with every passing day.");
        wait();

        Self {
            party: Party::create(),
            map: Map::new(60, 30),
            time: Time::day_zero(),
            name_pool: NamePool::new(),
        }
    }

    fn load() -> Option<Self> {
        let result = read_to_string(SAVE_LOCATION).ok()?;
        let game = serde_json::from_str(&result).ok()?;

        println!("Savegame found.");
        pause();
        println!("Would you like to load it? (y/n)");
        println!("If you select no, the savegame will be erased.");
        loop {
            print!(": ");
            flush();

            match read_line().to_lowercase().trim() {
                "y" | "yes" => return game,
                "n" | "no" => return None,
                _ => println!("Invalid option."),
            }
        }
    }

    pub fn load_or_new() -> Self {
        match Self::load() {
            Some(game) => game,
            None => Self::new(),
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string(self).expect("serialize to succeed");
        write(SAVE_LOCATION, json).expect("write to succeed");
    }

    pub fn run(&mut self) {
        loop {
            self.party
                .display_options(&mut self.time, &mut self.map, &mut self.name_pool);
            self.save();

            if self.party.check_failure() {
                let _ = remove_file(SAVE_LOCATION);
                println!("As the last member collapses to the ground, the surrounding area grows quiet once again...");
                wait();
                clear();
                println!("Final stats for this run:\n{}", self.party);
                println!("\nMap:\n\n{}", self.map);
                wait();
                break;
            }
        }
    }
}
