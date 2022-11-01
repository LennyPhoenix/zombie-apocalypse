mod event;
mod lib;
mod map;
mod member;
mod party;
mod time;

use map::Map;
use party::Party;
use time::Time;

fn main() {
    let mut party = Party::create();
    let mut map = Map::new(60, 30);
    let mut time = Time::first_morning();
    loop {
        party.display_options(&mut time, &mut map);
        if party.check_failure() {
            println!("Final stats for this run:\n{}", party);
        }
    }
}
