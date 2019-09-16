use std::env;

fn main() {
    	let args: Vec<String> = env::args().collect();
    	
	// for all given arguments
	for i in &args { 
		// debug print out all arguments
		//println!("{}", i); 
		match i.as_ref() {
			"measure_clock" => println!("1500000"),
			"measure_volts" => println!("1.2"),
			"measure_temp" => measure_temp(),
			_ => print!(""),
		}
	}

	
	
}

fn measure_temp() {
	println!("45.2");
}
