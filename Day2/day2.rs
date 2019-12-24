use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryFrom;

fn main() {
    let path = Path::new("input1.txt");

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
			let current_value = get_value(index, &num_vec);
			while current_value != 99u32
			{
				match current_value {
					1u32 => {
						println!("Add!");
						let first_arg = get_value(index + 1, &num_vec);
						let second_arg = get_value(index + 2, &num_vec);
						let final_pos = usize::try_from(get_value(index + 3, &num_vec)).unwrap();
						set_value(final_pos, (first_arg + second_arg).to_string(), &mut num_vec);
						index += 4;
					},
					2u32 => { 
						println!("Multiply!");
						let first_arg = get_value(index + 1, &num_vec);
						let second_arg = get_value(index + 2, &num_vec);
						let final_pos = usize::try_from(get_value(index + 3, &num_vec)).unwrap();
						set_value(final_pos, (first_arg * second_arg).to_string(), &mut num_vec);
						index += 4;
					},
					_ => {
						panic!("Unknown opcode {}, exiting", current_value);
					},
				}
			}
			println!("Final string: {}", s);
		}
    }
}

fn get_value(index : usize, num_vec : &Vec<String>) -> u32
{
	return num_vec[index].parse::<u32>().unwrap();
}

fn set_value(index : usize, value : String, num_vec : &mut Vec<String>)
{
	num_vec.remove(index);
	num_vec.insert(index, value);
}
