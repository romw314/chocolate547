fn main() {
	println!("BOOOOO! This is a Ghost House! Can you find the exit? Eee he heeee... eee he heeee...");
	print!("BOOOOO!");
	for arg in std::env::args().skip(1) {
		print!(" {}!", arg);
	}
	println!();
}
