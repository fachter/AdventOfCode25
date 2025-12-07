pub mod days;
pub mod utils;

use crate::days::{day03::run_battery_calculations, day07::run_laboratory};

fn main() {
    run_battery_calculations();
    run_laboratory();
    // crate::days::day03::run_battery_calculations();
}
