use std::env;

// see also https://elinux.org/RPI_vcgencmd_usage

fn main() {
	let args: Vec<String> = env::args().collect();
	//println!("{}", args.len());
    	
	// at least one argument has been given
	if args.len() > 1 {


		let command = &args[1];
			// debug print out all arguments
			//println!("{}", i);
			match command.as_ref() {
				"measure_clock" => measure_clock(&args),
				"measure_volts" => measure_volts(),
				"measure_temp" => measure_temp(),
				_ => print!(""),
			}
		//}
	}

	
	
}

fn measure_clock(args: &Vec<String>) {

	let max: i32 = *&args.len() as i32;
	for i in args {
		//println!("{}", i);
	}

	println!("arm:    frequency(45)=700000000");
	println!("core:   frequency(1)=250000000");
	println!("h264:   frequency(28)=0");
	println!("isp:    frequency(42)=250000000");	
	println!("v3d:    frequency(43)=250000000");
	println!("uart:   frequency(22)=3000000");
	println!("pwm:    frequency(25)=0");
	println!("emmc:   frequency(47)=100000000");
	println!("pixel:  frequency(29)=154000000");
	println!("vec:    frequency(10)=0");
	println!("hdmi:   frequency(9)=163682000");
	println!("dpi:    frequency(4)=0");

}

fn measure_volts() {
	println!("core:   volt=1.20V");
	println!("sdram_c:        volt=1.20V");
	println!("sdram_i:        volt=1.20V");
	println!("sdram_p:        volt=1.20V");

}

fn measure_temp() {
	println!("temp=45.2'C");
}
