use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", why.description()),
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
						Ok(n) =>  sum = sum + ((n / 3.0).floor() - 2.0),
						Err(e) => panic!("bad conversion {}", e.description()),
					}
				}

			}
			println!("Total sum: {}", sum);
		}
    }
}