use rand::{seq::SliceRandom, thread_rng, Rng};

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
    Event::Zombie(5),
    Event::Zombie(5),
    Event::Nothing,
    Event::Nothing,
    Event::Fuel(1),
    Event::Fuel(1),
];

impl Event {
    pub fn roll(options: &[Event]) -> Self {
        let mut rng = thread_rng();

        let event = options
            .choose(&mut rng)
            .expect("size of options to be greater than 0")
            .clone();

        let sign = if rng.gen_bool(0.7) { 1 } else { -1 };

        match event {
            Event::Money(n) => Event::Money(rng.gen_range(1..=*n)),
            Event::Ammo(n) => Event::Ammo(rng.gen_range(1..=*n) * sign),
            Event::Fuel(n) => Event::Fuel(rng.gen_range(1..=*n)),
            Event::Zombie(n) => Event::Zombie(rng.gen_range(1..=*n)),
            Event::Food(n) => Event::Food(rng.gen_range(1..=*n) * sign),
            Event::Survivor(n) => Event::Survivor(rng.gen_range(0..=*n)),
            Event::Nothing => Event::Nothing,
        }
    }
}
