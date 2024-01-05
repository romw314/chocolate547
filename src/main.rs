use std::io::BufRead; // import the read_line function
use std::io::Write; // import the flush function

use clap::crate_version;

fn getnumstr(num: usize) -> &'static str {
	let last = num % 10;
	if last == 1 {
		return "st";
	}
	else if last == 2 {
		return "nd";
	}
	else if last == 3 {
		return "rd";
	}
	else {
		return "th";
	}
}

fn main() {
	println!("CHOCO v{}", crate_version!());

	let mut index = 1;
	loop {
		if let Some(arg) = std::env::args().nth(index) {
			println!("{}{} arg is {}", index, getnumstr(index), arg);
		}
		else {
			break;
		}
		index += 1;
	}
    println!("Hello!");
	println!("This is chocolate!");

	let mut input: String = Default::default();

	print!("What's chocolate? ");
	std::io::stdout().flush().unwrap(); // flush stdout
	let _ = std::io::stdin().lock().read_line(&mut input); // read line from stdin into input
	println!("Chocolate is {}!", input.trim());
}
