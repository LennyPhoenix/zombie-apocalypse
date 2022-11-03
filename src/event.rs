use std::cmp::max;

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    io::{ellipsis, pause, wait},
    member::{Member, NamePool},
    party::Party,
};

pub enum Event {
    Ammo(i32),
    Food(i32),
    Money(i32),
    Fuel(i32),
    Zombie(i32),
    Survivor(i32),
    Nothing,
}

pub const DEFAULT_DAY_OPTIONS: &[Event] = &[
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
];

pub const DEFAULT_NIGHT_OPTIONS: &[Event] = &[
    Event::Zombie(5),
    Event::Zombie(5),
    Event::Nothing,
    Event::Fuel(1),
];

impl Event {
    pub fn roll(options: &[Event]) -> Self {
        let mut rng = thread_rng();

        let event = options
            .choose(&mut rng)
            .expect("size of options to be greater than 0");

        let sign = if rng.gen_bool(0.7) { 1 } else { -1 };

        match *event {
            Event::Money(n) => Event::Money(rng.gen_range(1..=n)),
            Event::Ammo(n) => Event::Ammo(rng.gen_range(1..=n) * sign),
            Event::Fuel(n) => Event::Fuel(rng.gen_range(1..=n)),
            Event::Zombie(n) => Event::Zombie(rng.gen_range(1..=n)),
            Event::Food(n) => Event::Food(rng.gen_range(1..=n) * sign),
            Event::Survivor(n) => Event::Survivor(rng.gen_range(0..=n)),
            Event::Nothing => Event::Nothing,
        }
    }

    pub fn handle(&self, party: &mut Party, name_pool: &mut NamePool) {
        match *self {
            Event::Money(amount) => money(amount, party),
            Event::Ammo(amount) => ammo(amount, party),
            Event::Fuel(amount) => fuel(amount, party),
            Event::Food(amount) => food(amount, party),
            Event::Zombie(amount) => zombie(amount, party),
            Event::Survivor(amount) => survivor(amount, party, name_pool),
            Event::Nothing => nothing(),
        }
    }
}

fn nothing() {
    println!("As you walk on, you realise you have lost track of the mystery machine.");
    pause();
    print!("You spend the next hour finding your place on the map, the situation slowly chipping away at your sanity");
    ellipsis();
    println!();
}

fn survivor(amount: i32, party: &mut Party, name_pool: &mut NamePool) {
    let mut rng = thread_rng();

    print!("In the distance, you spot a column of smoke. The party rushes towards it");
    ellipsis();
    println!();
    if amount > 0 {
        if amount == 1 {
            println!(
                "There is a lone survivor waiting by a campfire, they happily join your party."
            );
        } else {
            println!("There are a group of survivors huddled by the campfire, they happily join your party.");
        }
        for _ in 0..amount {
            wait();
            let member = Member::new(
                &name_pool.get(),
                rng.gen_range(10..=20),
                rng.gen_range(0..=5),
                Some(rng.gen_range(8..=10)),
            );
            println!("You are joined by {}", member);
            party.members.insert(0, member);
        }
    } else {
        print!("You finally reach the campfire, but there is nothing here but a few corpses");
        ellipsis();
        println!("\nThe bodies are still warm.");
    }
}

fn zombie(amount: i32, party: &mut Party) {
    if amount > 1 {
        println!("While searching you hear some groaning nearby, and turn around to see {} zombies lunge towards you!", amount);
    } else {
        println!("While searching you hear some groaning nearby, and turn around to see a zombie lurching towards you!");
    }
    pause();
    party.combat(amount);
}

fn money(amount: i32, party: &mut Party) {
    print!("You stumble across a corpse. It looks safe to search");
    ellipsis();
    println!("\n+{} money", amount);
    if thread_rng().gen_bool(0.3) {
        pause();
        let mut member = party.members.pop().expect("members");
        member.infection_level += 10;
        println!(
            "{} feels a little dizzy after leaving the corpse...",
            member.name
        );
        party.members.insert(0, member);
    }
    party.money += amount;
}

fn ammo(amount: i32, party: &mut Party) {
    if amount > 0 {
        println!(
            "You manage to break into a park ranger's locker, but there are only a few shells."
        );
        pause();
        println!("Whoever was last here took as much as they could...");
        pause();
        println!("+{} ammo", amount);
    } else if party.ammo > 0 {
        println!("You hear some rustling in some bushes dead ahead of you.");
        pause();
        print!("You fire your shotgun at it out of impluse");
        ellipsis();
        println!("\nIt was only a few rats.");
        pause();
        println!("{} ammo", amount);
    } else {
        print!("You hear some rustling in the bushes ahead of you. The dread sets in");
        ellipsis();
        println!();
    }
    party.ammo += amount;
}

fn fuel(amount: i32, party: &mut Party) {
    print!("You stumble across someone's derelect house. The owners are long gone");
    ellipsis();
    println!("\nThere is nothing here, save for a half-empty jerry can of petrol.");
    pause();
    println!("+{} fuel", amount);
    party.fuel += amount;
}

fn food(amount: i32, party: &mut Party) {
    if amount > 0 {
        print!("You stumble across someone's derelect house. The owners are long gone");
        ellipsis();
        println!("\nThere is still some old food in the fridge.");
        pause();
        println!("+{} food", amount);
    } else if party.food > 0 {
        let n = max(amount, -party.food);
        println!("While searching the area, some of your food spoils and is made inedible...");
        pause();
        println!("{} food", n);
    }
    party.food += amount;
}
