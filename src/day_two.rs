pub fn intcode_computer(noun: u32, verb: u32) -> u32 {
    let intcode = format!("1,{},{},3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,6,23,27,1,27,5,31,2,9,31,35,1,5,35,39,2,6,39,43,2,6,43,47,1,5,47,51,2,9,51,55,1,5,55,59,1,10,59,63,1,63,6,67,1,9,67,71,1,71,6,75,1,75,13,79,2,79,13,83,2,9,83,87,1,87,5,91,1,9,91,95,2,10,95,99,1,5,99,103,1,103,9,107,1,13,107,111,2,111,10,115,1,115,5,119,2,13,119,123,1,9,123,127,1,5,127,131,2,131,6,135,1,135,5,139,1,139,6,143,1,143,6,147,1,2,147,151,1,151,5,0,99,2,14,0,0", noun, verb);
    let mut program: Vec<u32> = intcode.split(",").map(|s| s.parse().unwrap()).collect();
    let mut instruction_pointer = 0;

    loop {
        let instruction = program[instruction_pointer];

        if instruction == 1 {
            let first_number: u32 = program[program[instruction_pointer + 1] as usize];
            let second_number: u32 = program[program[instruction_pointer + 2] as usize];
            let index = program[instruction_pointer + 3] as usize;

            program[index] = first_number + second_number;
        } else if instruction == 2 {
            let first_number: u32 = program[program[instruction_pointer + 1] as usize];
            let second_number: u32 = program[program[instruction_pointer + 2] as usize];
            let index = program[instruction_pointer + 3] as usize;

            program[index] = first_number * second_number;
        } else if instruction == 99 {
            break;
        } else {
            println!("\n\n\n\nILLEGIAL instruction -> {}\n\n\n\n", instruction);
            break;
        }

        instruction_pointer = instruction_pointer + 4
    }

    program[0]
}

const MONEY: u32 = 19690720;

pub fn gravity_assist() -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            if intcode_computer(noun, verb) == MONEY {
                return (100 * noun) + verb;
            }
        }
    }

    return 0;
}
