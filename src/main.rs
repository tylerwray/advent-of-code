fn main() {
    day_one_part_one();
    day_one_part_two();
}

const FUEL_MASSES: [u32; 100] = [
    54172, 58469, 92948, 143402, 57563, 54532, 68042, 89847, 70872, 54069, 107310, 146439, 88851,
    142869, 71309, 89613, 70338, 87708, 95305, 134384, 128250, 134991, 91270, 127819, 68650,
    102556, 129882, 68688, 129939, 137344, 102624, 90828, 86487, 91712, 114866, 75697, 107599,
    99053, 87511, 128128, 57772, 69314, 90771, 145376, 100730, 142675, 112731, 83985, 123565,
    127325, 86597, 121772, 131992, 148859, 93348, 77294, 119763, 74636, 95592, 79628, 78861, 68565,
    88820, 134291, 69262, 128678, 118216, 52799, 92731, 61600, 63477, 64016, 131872, 131412,
    146579, 104400, 99110, 63458, 144393, 54787, 148622, 91323, 61137, 106082, 103644, 63795,
    126648, 61489, 140964, 110963, 72696, 124370, 110466, 139317, 108440, 148062, 89992, 145645,
    70556, 95739,
];

fn day_one_part_one() {
    let mut total_mass = 0;

    for mass in FUEL_MASSES.iter() {
        total_mass += (mass / 3) - 2
    }

    println!("Total mass is: {}", total_mass)
}

fn day_one_part_two() {
    let mut total_mass = 0;

    for mass in FUEL_MASSES.iter() {
        total_mass += calc_fuel_mass(*mass);
    }

    println!("Total mass, plus fuel mass, is: {}", total_mass)
}

fn calc_fuel_mass(initial_mass: u32) -> u32 {
    let mut total_mass = 0;

    let mut rem_mass = initial_mass;

    loop {
        let tester = rem_mass / 3;
        if tester <= 2 {
            break;
        }
        rem_mass = tester - 2;
        total_mass += rem_mass;
    }

    total_mass
}
