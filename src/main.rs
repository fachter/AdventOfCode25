pub mod days;
pub mod utils;

use crate::days::{
    // day03::run_battery_calculations,
    // day04::run_print_optimization,
    day05::run_food_expiration,
    day08::run_playground,
    // day07::run_laboratory,
};

fn main() {
    //run_battery_calculations();
    // run_print_optimization();
    run_food_expiration();
    run_playground();
    //run_laboratory();
}
