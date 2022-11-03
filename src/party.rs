use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    cmp::{max, min},
    fmt::Display,
};

use crate::{
    event::{Event, DEFAULT_DAY_OPTIONS, DEFAULT_NIGHT_OPTIONS},
    io::{clear, ellipsis, flush, pause, read_line, wait},
    map::{Direction, Map},
    member::{DeathCheckResult, Member, NamePool},
    time::Time,
};

const TRAVEL_COST: i32 = 1;
const TRAVEL_TIME: i32 = 6;

#[derive(Serialize, Deserialize)]
pub struct Party {
    pub ammo: i32,
    pub money: i32,
    pub fuel: i32,
    pub medicine: i32,
    pub food: i32,
    pub members: Vec<Member>,
}

impl Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- Ammo: {}\n- Money: {}\n- Fuel: {}\n- Food: {}\n- Medicine: {}",
            self.ammo, self.money, self.fuel, self.food, self.medicine
        )
    }
}

impl Party {
    pub fn create() -> Self {
        let mut rng = thread_rng();
        let money = rng.gen_range(4..=6);
        let ammo = rng.gen_range(4..=5);

        let member_1 = Party::choose_character();
        let member_2 = Member::scoob();

        print!(
            "{name} is able to locate the mystery machine",
            name = member_1.name
        );
        ellipsis();
        println!("\n{name} is still inside.", name = member_2.name);

        let party = Self {
            ammo,
            money,
            food: 4,
            medicine: 0,
            fuel: 2,
            members: vec![member_1, member_2],
        };

        for member in party.members.iter() {
            println!("{}\n", member);
        }
        wait();

        party
    }

    pub fn normalise_values(&mut self) {
        self.food = max(0, self.food);
        self.ammo = max(0, self.ammo);
        self.medicine = max(0, self.medicine);
        self.money = max(0, self.money);
    }

    pub fn combat(&mut self, mut zombie_count: i32) {
        let mut rng = thread_rng();
        // Shuffle Member List
        self.members.shuffle(&mut rng);

        let used_ammo = min(rng.gen_range(0..=self.ammo), zombie_count);

        if used_ammo == zombie_count {
            if zombie_count > 1 {
                println!("You quickly drew your shotgun and managed to kill all the zombies, you were lucky this time.");
            } else {
                println!("You quickly drew your shotgun and managed to kill the attacking zombie, you were lucky this time.");
            }
        } else if used_ammo > 0 {
            if used_ammo > 1 {
                println!("You quickly drew your shotgun and managed to kill {} zombies before they could attack.", used_ammo);
            } else {
                println!("You quickly drew your shotgun and managed to kill one zombie before it could attack.");
            }
            pause();
            println!("The rest lurch towards the party...");
        } else if used_ammo == 0 && self.ammo > 0 {
            println!("You had ammo to your disposal, but were not able to draw your weapons quick enough to attack the zombies...");
        }

        zombie_count -= used_ammo;
        self.ammo -= used_ammo;

        wait();

        while zombie_count > 0 && self.members.len() > 0 {
            let mut member = self
                .members
                .pop()
                .expect("length of members to be greater than 0");

            let attacking = rng.gen_range(1..=zombie_count);
            let damage = rng.gen_range(attacking..=(attacking * 2));

            if attacking > 1 {
                print!("{} zombies attack {}", attacking, member.name);
            } else {
                print!("A zombie attacks {}", member.name);
            }
            ellipsis();
            println!("");

            member.hurt(damage);

            pause();

            match member.check_dead() {
                DeathCheckResult::Alive => {
                    let retaliation = rng.gen_range(1..=attacking);
                    if retaliation > 1 {
                        println!(
                            "{} manages to neutralise {retaliation} of the zombies.",
                            member.name
                        );
                    } else {
                        println!("{} manages to neutralise a zombie.", member.name);
                    }
                    zombie_count -= retaliation;
                    self.members.insert(0, member);
                }
                DeathCheckResult::Dead => {
                    print!("{} collapses to the ground, the zombies are", member.name);
                    ellipsis();
                    println!(" occupied.");
                    pause();
                    println!("For now.");
                    zombie_count -= attacking;
                }
                DeathCheckResult::Undead => {
                    print!(
                        "A horrendous crunch is heard, and {} collapses to the ground",
                        member.name
                    );
                    ellipsis();
                    println!("\nA shriek fills the air and {} begins crawling towards the rest of the party...", member.name);
                    zombie_count += 1;
                }
            };

            pause();

            if zombie_count > 1 {
                println!("There are now {} zombies left...", zombie_count);
            } else if zombie_count == 1 {
                println!("1 zombie remains...");
            }
            wait();
        }

        if !self.check_failure() {
            println!("The attackers have been defeated...");
        } else {
            return;
        }
    }

    fn choose_character() -> Member {
        loop {
            println!("Please choose a character by selecting their number: ");
            println!("(1) Velma");
            println!("(2) Shaggy");
            println!("(3) Fred");
            println!("(4) Daphne");
            print!(": ");
            flush();

            let input = read_line();

            match input.trim() {
                "1" => return Member::velma(),
                "2" => return Member::shaggy(),
                "3" => return Member::fred(),
                "4" => return Member::daphne(),
                _ => println!("Invalid option."),
            }
        }
    }

    pub fn check_infection(&mut self) {
        let mut zombies = 0;
        for _ in 0..self.members.len() {
            let mut member = self.members.pop().expect("members to be non-empty");
            match member.check_infection() {
                DeathCheckResult::Alive => self.members.insert(0, member),
                DeathCheckResult::Dead => {
                    print!("{} collapses on the ground, unmoving", member.name);
                    ellipsis();
                    println!("");
                }
                DeathCheckResult::Undead => {
                    zombies += 1;
                    print!(
                        "{} falls on the ground, and continues coughing",
                        member.name
                    );
                    ellipsis();
                    println!("");
                }
            }
        }
        if zombies > 0 {
            if zombies > 1 {
                println!("Suddenly, the {zombies} corpses leap from the ground and attack the rest of the party!");
            } else {
                println!(
                    "Suddenly, the corpse leaps from the ground and attacks the rest of the party!"
                );
            }
            pause();
            self.combat(zombies);
        }
    }

    pub fn check_failure(&self) -> bool {
        self.members.len() == 0
    }

    pub fn display_options(&mut self, time: &mut Time, map: &mut Map, name_pool: &mut NamePool) {
        clear();
        println!("{time}");
        println!("Party:\n{self}\n- Members: {}", self.members.len());

        loop {
            println!("\nWhat is your next action?");
            println!("(1) Show party members");
            println!("(2) Show map");
            println!("(3) Explore area");
            println!("(4) Feed party");
            println!("(5) Cure party");
            print!(": ");
            flush();

            let mut input = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut input).expect("valid input");

            println!("");

            match input.trim() {
                "1" => {
                    self.display_party_members();
                }
                "2" => self.show_map(time, map),
                "3" => self.search(time, map, name_pool),
                "4" => self.feed_party_members(),
                "5" => self.cure_party_members(),
                _ => {
                    println!("Invalid option.");
                    continue;
                }
            }

            break;
        }
    }

    fn show_map(&mut self, time: &mut Time, map: &mut Map) {
        loop {
            clear();
            println!("{time}");
            println!("{}", map);
            println!("Travel will cost {TRAVEL_COST} fuel and will take {TRAVEL_TIME} hours.");
            if self.fuel >= TRAVEL_COST {
                println!("You have {} fuel.", self.fuel);
                loop {
                    println!("Enter a compass direction to travel, or `back` to close the map.");
                    print!(": ");
                    flush();

                    let mut input = String::new();
                    let stdin = std::io::stdin();
                    stdin.read_line(&mut input).expect("valid input");

                    match input.to_lowercase().trim() {
                        "north" => map.travel(Direction::North),
                        "east" => map.travel(Direction::East),
                        "south" => map.travel(Direction::South),
                        "west" => map.travel(Direction::West),
                        "back" => return,
                        _ => {
                            println!("Invalid option.");
                            continue;
                        }
                    }

                    time.advance(TRAVEL_TIME);
                    self.fuel -= TRAVEL_COST;
                    print!("The party packs into the mystery machine, and you spend the next {TRAVEL_TIME} hours travelling");
                    ellipsis();
                    ellipsis();
                    println!("");
                    break;
                }
            } else {
                println!("You do not have enough fuel to travel.");
                wait();
                return;
            }
        }
    }

    fn cure_party_members(&mut self) {
        loop {
            clear();
            for (num, member) in self.members.iter().enumerate() {
                println!("({num}) {member}\n", num = num + 1);
            }
            println!("You have {} medicine.", self.medicine);

            loop {
                println!("Enter a number to attempt to cure a party member, or 'back' to exit this menu.");

                print!(": ");
                flush();
                let mut input = String::new();
                let stdin = std::io::stdin();
                stdin.read_line(&mut input).expect("valid input");
                let input = input.trim().to_lowercase();

                if input == "back" {
                    return;
                }

                let choice = match input.parse::<usize>() {
                    Ok(choice) => choice,
                    Err(_) => 0,
                };

                if choice > 0 && choice <= self.members.len() {
                    let member = self
                        .members
                        .get_mut(choice - 1)
                        .expect("choice to be in bounds");

                    if self.medicine > 0 {
                        print!("{} takes some of the antibiotics", member.name);
                        ellipsis();
                        println!("\nThey feel slightly better now.");
                        member.cure(thread_rng().gen_range(5..=10));
                        self.medicine -= 1;
                        wait();
                        break;
                    } else {
                        println!(
                            "You do not have enough medicine for {} to use...",
                            member.name
                        );
                    }
                } else {
                    println!("Invalid input.");
                }
            }
        }
    }

    fn feed_party_members(&mut self) {
        loop {
            clear();
            for (num, member) in self.members.iter().enumerate() {
                println!("({num}) {member}\n", num = num + 1);
            }
            println!("You have {} food.", self.food);

            loop {
                println!("Enter a number to feed a party member, or 'back' to exit this menu.");

                print!(": ");
                flush();
                let mut input = String::new();
                let stdin = std::io::stdin();
                stdin.read_line(&mut input).expect("valid input");
                let input = input.trim().to_lowercase();

                if input == "back" {
                    return;
                }

                let choice = match input.parse::<usize>() {
                    Ok(choice) => choice,
                    Err(_) => 0,
                };

                if choice > 0 && choice <= self.members.len() {
                    let member = self
                        .members
                        .get_mut(choice - 1)
                        .expect("choice to be in bounds");

                    if self.food > 0 {
                        print!("{} begins eating", member.name);
                        ellipsis();
                        println!("\nThey feel slightly better now.");
                        member.heal(thread_rng().gen_range(2..=4));
                        self.food -= 1;
                        wait();
                        break;
                    } else {
                        println!("You do not have enough food for {} to eat...", member.name);
                    }
                } else {
                    println!("Invalid input.");
                }
            }
        }
    }

    fn display_party_members(&self) {
        clear();
        for member in self.members.iter() {
            println!("{}\n", member);
        }
        wait();
    }

    fn search(&mut self, time: &mut Time, map: &mut Map, name_pool: &mut NamePool) {
        let tile = map.get_tile(None);

        clear();
        println!("{}", time);
        println!("Party:\n{}\n- Members: {}\n", self, self.members.len());

        match &tile.location_type {
            // Special Location
            Some(location) if !tile.explored => location.handle(self, name_pool),
            // Normal Tile
            _ => {
                let options = if time.night() {
                    DEFAULT_NIGHT_OPTIONS
                } else {
                    DEFAULT_DAY_OPTIONS
                };

                // TODO: Location specific options
                Event::roll(options).handle(self, name_pool);
            }
        }

        self.normalise_values();
        time.advance(1);

        if !self.check_failure() {
            self.check_infection();
            wait();
        }

        map.explore();
    }
}
