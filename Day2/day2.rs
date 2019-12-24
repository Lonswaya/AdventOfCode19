use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    //solve_part_1();
	solve_part_2();
}

fn get_value(index : usize, num_vec : &Vec<String>) -> u32
{
	let result = &num_vec[index];
	return result.parse::<u32>().unwrap();
}

fn set_value(index : usize, value : String, num_vec : &mut Vec<String>)
{
	num_vec.remove(index);
	num_vec.insert(index, value);
}


fn solve_part_1()
{
	let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };
	
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => 
		{
			let mut index = 0;
			let mut num_vec: Vec<String> = s.split(',')
											.map(|c| c.to_string())
											.collect();
			let mut current_value = get_value(index, &num_vec);
			while current_value != 99u32
			{
			//println!("index: {}, value: {}", index.to_string(), current_value);
				match current_value {
					1u32 => {
						//println!("Add!");
						let first_arg = get_value(get_value(index + 1, &num_vec) as usize, &num_vec);
						let second_arg = get_value(get_value(index + 2, &num_vec) as usize, &num_vec);
						let final_pos = get_value(index + 3, &num_vec) as usize;
						set_value(final_pos, (first_arg + second_arg).to_string(), &mut num_vec);
						index += 4;
					},
					2u32 => { 
						//println!("Multiply!");
						let first_arg = get_value(get_value(index + 1, &num_vec) as usize, &num_vec);
						let second_arg = get_value(get_value(index + 2, &num_vec) as usize, &num_vec);
						let final_pos = get_value(index + 3, &num_vec) as usize;
						set_value(final_pos, (first_arg * second_arg).to_string(), &mut num_vec);
						index += 4;
					},
					_ => {
						panic!("Unknown opcode {}, exiting", current_value);
					},
				}
				current_value = get_value(index, &num_vec);
			}
			let mut final_string = String::new();
			for st in num_vec
			{
				let mut other_string = st;
				&other_string.push(',');
				final_string.push_str(&other_string);
			}
			println!("Final string: {}", final_string);
		}
    }
}

fn solve_part_2()
{
	let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };
	let magic_number = 19690720;
	
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => 
		{
			let s_copy = s;

			for first_guess in 0..99 
			{
				for second_guess in 0..99 
				{
					let final_value = solve(&s_copy, first_guess, second_guess);
					if final_value == magic_number
					{
						println!("Magic number found! x: {}, y: {}", first_guess, second_guess);
						break;
					}
					//println!("Attempting: x: {}, y: {}, Result: {}", first_guess, second_guess, final_value);
				}
			}
		}
    }
}

fn solve(s : &String, first_guess : usize, second_guess : usize) -> u32
{
	let mut index = 0;
	let mut num_vec: Vec<String> = s.split(',')
									.map(|c| c.to_string())
									.collect();
	let mut current_value = get_value(index, &num_vec);
	set_value(1usize, first_guess.to_string(), &mut num_vec);
	set_value(2usize, second_guess.to_string(), &mut num_vec);
	while current_value != 99u32
	{
	//println!("index: {}, value: {}", index.to_string(), current_value);
		match current_value {
			1u32 => {
				//println!("Add!");
				let first_arg = get_value(get_value(index + 1, &num_vec) as usize, &num_vec);
				let second_arg = get_value(get_value(index + 2, &num_vec) as usize, &num_vec);
				let final_pos = get_value(index + 3, &num_vec) as usize;
				set_value(final_pos, (first_arg + second_arg).to_string(), &mut num_vec);
				index += 4;
			},
			2u32 => { 
				//println!("Multiply!");
				let first_arg = get_value(get_value(index + 1, &num_vec) as usize, &num_vec);
				let second_arg = get_value(get_value(index + 2, &num_vec) as usize, &num_vec);
				let final_pos = get_value(index + 3, &num_vec) as usize;
				set_value(final_pos, (first_arg * second_arg).to_string(), &mut num_vec);
				index += 4;
			},
			_ => {
				panic!("Unknown opcode {}, exiting", current_value);
			},
		}
		current_value = get_value(index, &num_vec);
	}
	return num_vec[0usize].parse::<u32>().unwrap()
}