use std::cmp::min;

use rand::{seq::IteratorRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    io::{ellipsis, flush, pause, read_line, wait},
    member::{Member, NamePool},
    party::Party,
};

#[derive(EnumIter, Serialize, Deserialize)]
pub enum Location {
    ShoppingCentre,
    TradeWell,
    MilitaryBase,
}

impl Location {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::iter()
            .choose(&mut rng)
            .expect("location to be non-empty")
    }

    pub fn handle(&self, party: &mut Party, name_pool: &mut NamePool) {
        match self {
            Location::ShoppingCentre => shopping_centre(party, name_pool),
            Location::TradeWell => trade_well(party),
            Location::MilitaryBase => military_base(party, name_pool),
        }
    }
}

fn military_base(party: &mut Party, name_pool: &mut NamePool) {
    print!("You seem to have parked just outside an old military base");
    ellipsis();
    println!("\nThere are no guards in sight.");
    pause();
    println!("You enter the building...");
    wait();
    println!("You encounter a small room with a large blast door to one side.");
    pause();
    println!("A small panel to the bottom right says \"Emergency lockdown, do not open without higher approval.\"");
    pause();
    println!("Despite the warning, there is a key sitting on the control panel.");
    pause();
    loop {
        println!("Do you open the door? (y/n)");
        print!(": ");
        flush();

        let input = read_line();

        match input.to_lowercase().trim() {
            "y" | "yes" => {
                print!("You turn the key in the control panel");
                ellipsis();
                println!();
                if thread_rng().gen_bool(0.4) {
                    let num = thread_rng().gen_range(1..=2);
                    if num > 1 {
                        println!("{num} survivors rush out of the room, gasping for fresh air.");
                    } else {
                        println!("A survivor rushes out of the room, gasping for fresh air.");
                    }
                    pause();
                    for _ in 0..num {
                        let member = Member::new(
                            &name_pool.get(),
                            thread_rng().gen_range(18..=30),
                            thread_rng().gen_range(0..=5),
                            Some(thread_rng().gen_range(10..=18)),
                        );
                        println!("You are joined by {}", member);
                        wait();
                        party.members.push(member);
                    }
                    println!("They explain that they had locked themselves in the bunker for safety, and thank you for releasing them.");
                } else {
                    let num = thread_rng().gen_range(4..=5);
                    println!("Not a second after the door opens, {num} zombies leap from the room and attack you!");
                    party.combat(num);
                    if party.check_failure() {
                        return;
                    }

                    print!("You trudge on, slightly on-edge after that encounter");
                    ellipsis();
                    println!();
                }
            }
            "n" | "no" => {
                print!("You continue onwards, its not worth the risk");
                ellipsis();
                println!();
            }
            _ => {
                println!("Invalid input.");
                continue;
            }
        }
        break;
    }
    wait();
    print!("You stumble across a weapons locker");
    ellipsis();
    println!("\nAll weapons have been taken, but there is still plenty of ammo.");
    pause();
    let amount = thread_rng().gen_range(5..=9);
    println!("+{amount} ammo");
    party.ammo += amount;
    wait();
    println!("There is also a small box of unlabeled tablets.");
}

fn trade_well(party: &mut Party) {
    println!("As you step out of the mystery machine, you spot a small well nearby.");
    pause();
    print!("There is a small bucket hanging from a frayed-looking rope");
    ellipsis();
    println!("\nIt looks like there is a small $ sign scratched onto the surface of the bucket.");
    pause();
    print!("You call out into the well");
    ellipsis();
    println!(" Vague shuffling is heard, but no-one responds.");
    wait();
    let is_medicine = thread_rng().gen_bool(0.7);
    let initial_amount = thread_rng().gen_range(2..=3);
    if is_medicine {
        println!("Looking inside the bucket, there are a couple tablets of medicine.");
        pause();
        println!("+{initial_amount} medicine");
        party.medicine += initial_amount;
    } else {
        println!("Looking inside the bucket, there are a couple shotgun shells.");
        pause();
        println!("+{initial_amount} ammo");
        party.ammo += initial_amount;
    }
    wait();
    loop {
        println!("You have {} money.", party.money);
        pause();
        print!("How much money do you put into the bucket? ");
        flush();

        let input = read_line();
        let amount = input.trim().parse::<i32>().unwrap_or(-1);

        if amount < 0 || amount > party.money {
            println!("Invalid choice.");
            continue;
        } else if amount == 0 {
            println!("You leave the bucket.");
        } else {
            party.money -= amount;
            print!("Almost immediately after placing the money in the bucket, it begins descending down into the dark below");
            ellipsis();
            print!("\nJust as you begin to think whoever is down there has just taken your money and left, the bucket begins rising back up again");
            ellipsis();
            println!();
            if is_medicine {
                if amount > 1 {
                    println!("In the bucket, there are exactly {amount} loose antibiotic tablets, as expected.");
                } else {
                    println!("In the bucket, is a single antibiotic tablet, as expected.");
                }
                pause();
                party.medicine += amount;
                println!("+{amount} medicine");
            } else {
                if amount > 1 {
                    println!("In the bucket there are exactly {amount} shells, as expected.");
                } else {
                    println!("In the bucket, is a single shotgun shell, as expected.");
                }
                party.ammo += amount;
                println!("+{amount} ammo");
            }
            pause();
            println!("-{amount} money");
        }

        break;
    }
    wait();

    print!("As soon as you turn around from the well the rope snaps,");
    flush();
    pause();
    print!(" leaving the bucket to fall down into the well with a crash");
    ellipsis();
    println!("\nAn anguished screech is heard from inside the well.");
    pause();
    println!("You decide not to stick around.");
}

fn shopping_centre(party: &mut Party, name_pool: &mut NamePool) {
    println!("You step out of the mystery machine to discover you have parked just outside an old shopping centre.");
    pause();
    print!("The party begins to explore the building");
    ellipsis();
    let food = thread_rng().gen_range(4..=8);
    party.food += food;
    println!("\nAfter looting what was left of the shops, you manage to salvage some food!");
    pause();
    println!("+{food} food");
    wait();
    print!("You continue searching");
    ellipsis();
    println!("\nThe party discovers a small snack machine, it appears to still be working.");
    pause();
    // Snack Machine
    loop {
        print!(
            "You have {} money, how much would you like to pay into the machine? ",
            party.money
        );
        flush();

        let input = read_line();

        let amount = input.trim().parse::<i32>().unwrap_or(-1);

        if amount == 0 {
            print!("You leave the machine");
            ellipsis();
            println!("\nAs you walk away, the power in the building shuts off.");
            pause();
        } else if amount < 0 || amount > party.money {
            println!("Invalid amount.");
            continue;
        } else {
            if amount > 1 {
                print!("You begin putting {amount} coins into the machine");
            } else {
                print!("You put a coin into the machine");
            }
            ellipsis();
            println!();
            let limit = thread_rng().gen_range(5..=7);
            let food_amount = min(limit, amount);
            let spent = min(limit + 1, amount);
            if food_amount > 1 {
                print!("One by one, the machine spits out {food_amount} small snack items");
            } else {
                print!("The machine spits out a small snack item");
            }
            ellipsis();
            println!(" Its not much, but it will do.");
            pause();
            if spent > limit {
                println!("As you put another coin into the machine, the power shuts off.");
                pause();
                print!("The coin is stuck somewhere in the iternals of the machine, ");
                flush();
                pause();
                println!("and you give up trying to retrieve it.");
            } else {
                println!(
                    "As the last item leaves the machine, the power in the building shuts off."
                );
            }
            pause();
            party.food += food_amount;
            party.money -= spent;
            println!("+{food_amount} food");
            pause();
            println!("-{spent} money");
            wait();
        }
        break;
    }
    print!("Its very dark now");
    ellipsis();
    println!(" The building's power reserves must have finally been exausted.");
    pause();
    println!("Suddenly, a terrible shriek is heard from another part of the building.");
    pause();
    loop {
        println!("Do you attempt to locate it? (y/n)");
        print!(": ");
        flush();

        let input = read_line().to_lowercase();
        match input.trim() {
            "y" | "yes" => {
                print!("You rush towards the sound");
                ellipsis();
                println!();
                let mut zombies = thread_rng().gen_range(4..=6);
                if thread_rng().gen_bool(0.5) {
                    println!(
                        "Through the dark, you spot someone sprinting away from a few zombies..."
                    );
                    pause();
                    let member = Member::new(
                        &name_pool.get(),
                        thread_rng().gen_range(10..=20),
                        thread_rng().gen_range(10..=15),
                        Some(thread_rng().gen_range(6..=10)),
                    );
                    println!("{member}");
                    party.members.push(member);
                } else {
                    zombies += 1;
                    println!("Through the dark, you spot a mob of {zombies} zombies lurching around the building.");
                    pause();
                    print!("You were too late");
                    ellipsis();
                    println!("\nSuddenly, you are spotted, and the mob lunges towards you...");
                }
                wait();
                party.combat(zombies);

                if party.check_failure() {
                    return;
                }
            }
            "n" | "no" => {
                println!("Its not worth it.");
            }
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
        break;
    }

    print!("You leave the shopping centre");
    ellipsis();
    println!();
}
