mod day_one;
mod day_three;
mod day_two;

fn main() {
    day_one::check_module_mass();
    day_one::check_fuel_mass();

    let position_zero = day_two::intcode_computer(12, 2);
    println!("Position 0 -> {}", position_zero);

    let code = day_two::gravity_assist();
    println!("Code -> {}", code);

    let distance = day_three::distance_to_closest_crossing_point();
    println!("Distance -> {}", distance);
}
