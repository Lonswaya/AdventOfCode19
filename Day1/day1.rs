use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
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
			let mut sum = 0.0;
			let words: Vec<&str> = s.split('\n').collect();
			for word in &words {
				if *word != "" {
				
					match word.parse::<f64>() {
						Ok(n) =>  sum = sum + fuel_calculator(n),
						Err(e) => panic!("bad conversion {}", e.description()),
					}
				}

			}
			println!("Total sum: {}", sum);
		}
    }
}

fn fuel_calculator(n : f64) -> f64 {
	let result = (n / 3.0).floor() - 2.0;
	if result <= 0.0 {
		return 0.0;
	} else {
		return result + fuel_calculator(result);
	}
}