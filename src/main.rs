//use std::env;

fn main() {
	/*
	   let args: Vec<String> = env::args().collect();
	   let number: i32 = args[1].parse().unwrap();

	   println!("|{}|", tens_pronunciation(number));
	   */
	for i in 0..1000{
		println!("| {} |  {}  ", i, hundreds_pronunciation(i));
	}
}


fn hundreds_pronunciation(x: i32) -> String{ 
	let mut string = String::new();	
	let hundreds = x / 100;
	let tens = x % 100;
	match hundreds {
		0 => string.push_str(&tens_pronunciation(tens)),
		1|2|3|4|5|6|7|8|9 => {
			string.push_str(&ones_pronunciation(hundreds));
			string.push_str(" hundred ");
			string.push_str(&tens_pronunciation(tens));
		},
		_ => panic!("Hundreds Error"),
	}
	string
} 

fn tens_pronunciation(x: i32) -> String{ 
	let mut string = String::new();	
	if x < 10 {
		string.push_str(&ones_pronunciation(x));
	}
	else if x >= 10 && x < 20 {
		string.push_str(&ten_to_nineteen(x));
	}
	else if x >= 20 && x <= 99{
		let ones = x % 10; 
		let tens = x / 10;
		string.push_str(&multiples_of_ten(tens));
		if ones > 0 {   
			string.push_str(" ");
			string.push_str(&ones_pronunciation(ones));
		}
	}	
	else {
		panic!("Tens Error: Number greater than 99");
	}
	string
} 

fn ones_pronunciation(x: i32) -> String{
	let mut string = String::new();	
	match x {
		0 => string.push_str(""),
		1 => string.push_str("one"),
		2 => string.push_str("two"),
		3 => string.push_str("three"),
		4 => string.push_str("four"),
		5 => string.push_str("five"),
		6 => string.push_str("six"),
		7 => string.push_str("seven"),
		8 => string.push_str("eight"),
		9 => string.push_str("nine"),
		_ => panic!("Ones Error"),
	}
	string
}

// helpers for tens pronunciation

fn ten_to_nineteen(x: i32) -> String{
	let mut string = String::new();	
	match x {
		10 => string.push_str("ten"),
		11 => string.push_str("eleven"),
		12 => string.push_str("twelve"),
		13 => string.push_str("thirteen"),
		14 => string.push_str("fourteen"),
		15 => string.push_str("fifteen"),
		16 => string.push_str("sixteen"),
		17 => string.push_str("seventeen"),
		18 => string.push_str("eighteen"),
		19 => string.push_str("nineteen"),
		_ => panic!("Tens Error: 10 through 19: {}", x),
	}
	string
}

fn multiples_of_ten(x: i32) -> String{
	let mut string = String::new();	
	match x {
		2 => string.push_str("twenty"),
		3 => string.push_str("thirty"),
		4 => string.push_str("fourty"),
		5 => string.push_str("fifty"),
		6 => string.push_str("sixty"),
		7 => string.push_str("seventy"),
		8 => string.push_str("eighty"),
		9 => string.push_str("ninety"),
		_ => panic!("Tens Error: Multiples of 10: {}", x),
	}
	string
}
