use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let string = &args[1];

	if string.contains(".") {
		panic!("Error: Floating point numbers are not supported yet.");
	}

	let number: i64 = string.parse().unwrap();

	if number < 0 {
		panic!("Error: Negative numbers are not supported yet.");
	}

	println!("{}", get_pronunciation(number));
}

// The way large number pronunciation occurs is as follows:
// Every large number is composed of 3 digit pronunciations. Meaning that
// whenever we're pronuncing a number that is very large, larger than thousands/
// millions/billions, we first pronounce the 3 digit number of the
// highest level significance, and then pronounce the significance
// level, which correspond to the powers of 1000 (e.g. thousand, million, billion etc).
// Later on, we move to the next 3 digits of highest significance level, and repeat the
// same thing.

// For example:
// Lets take the number 34,075,120. The highest level of significance for this number is
// million, and the 3 digit pronunciation at the million level is 'thirty four'. So we
// start pronuncing this number by saying 'thirty four million', and then we move on to
// the next highest significance level, which is thousand in this case. The 3 digit
// pronunciation in the thousands significance level for this number is 'seventy five',
// so we now say 'thirty four million, seventy five thousand'.

// One important edge case is that if a level of significance has 0 equivalent for it's
// 3 digit number, then we don't pronounce the name of the level of significance.
fn get_pronunciation(number: i64) -> String {
	let mut string = String::new();

	let power: i32 = (number as f64).log(1000_f64) as i32;
	let mut curr_number = number as i64;
	let mut curr_three_digit_number: i64;
	let mut curr_significance_level: i64;

	for curr_power in (0..power+1).rev() {

		curr_significance_level = 1000_i64.pow(curr_power as u32);
		curr_three_digit_number = curr_number / curr_significance_level;
		curr_number = curr_number - (curr_three_digit_number * curr_significance_level);

		if curr_three_digit_number > 0 {

			// To not print comma in the highest significance level
			if power != curr_power {
				string.push_str(", ");
			}

			// Do not print " "(space) and significance level if significance level
			// is less than thousand
			if curr_power != 0 {
				string.push_str(&get_three_digit_pronunciation(curr_three_digit_number));
				string.push_str(" ");
				string.push_str(&get_power_of_thousand_pronunciation(curr_power));
			}
			else {
				string.push_str(&get_three_digit_pronunciation(curr_three_digit_number));
			}
		}
	}
	string
}

// Three digit number pronunciation occurs as follows:
// If the number is greater than 99, then we first pronounce its hundreds digit
// as a number less than 10, add the word "hundred" to it. Later we follow up with
// the two digit pronunciation of the two other digits left.
// If the number is equal to or less than 99, we just pronounce the two digit
// pronunciation. Two digit pronunciation is explained above get_two_digit_pronunciation
// function.

// For example:
// Lets take the number 456. The hundreds digit for 456 is 4. So we pronounce, 'four',
// and we add 'hundred'. Later we just follow up with the two digit pronunciation,
// which is 'fifty six' in this case.
fn get_three_digit_pronunciation(x: i64) -> String{
	let mut string = String::new();	
	let hundreds_digit = x / 100;
	let tens_digit = x % 100;
	match hundreds_digit {
		0 => string.push_str(&get_two_digit_pronunciation(tens_digit)),
		1|2|3|4|5|6|7|8|9 => {
			string.push_str(&get_one_digit_pronunciation(hundreds_digit));
			string.push_str(" hundred");
			if tens_digit > 0 {
				string.push_str(" ");
				string.push_str(&get_two_digit_pronunciation(tens_digit));
			}
		},
		_ => panic!("Hundreds Error: x = {}", x),
	}
	string
} 

// Two digit pronunciation occurs as follows:
// There are specific cases for numbers 10 through 19, where these numbers naming
// don't hold to the convention seen when naming any other two digit number. Thus,
// we first check if the number we're trying to pronounce is one of these specific
// cases. If it is, we just use our specific cases helper to retrieve their pronunciation.// If the number is not 10 through 19, then it follows a naming convention as follows:
// We first pronounce the tens multiple pronounciation, such as twenty, fifty, sixty, and
// then if there is any ones digit, we pronounce that digit as a number less than 10.
fn get_two_digit_pronunciation(x: i64) -> String {
	let mut string = String::new();	
	if x >= 0 && x < 100 {
		// specific cases
		if x >= 10 && x < 20 {
			string.push_str(&get_specific_case_pronunciation(x));
		}
		else {
			let ones_digit = x % 10;
			let tens_digit = x / 10;
			if tens_digit > 0 {
				string.push_str(&get_multiple_of_ten_pronunciation(tens_digit));
			}
			if tens_digit > 0 && ones_digit > 0 {
				string.push_str(" ");
			}
			if ones_digit > 0 {
				string.push_str(&get_one_digit_pronunciation(ones_digit));
			}
		}
	}
	else {
		panic!("Two Digit Error: Number greater than 99 or less than 0: {}", x);
	}
	string
} 

// Returns pronunciation of the powers of thousand: 1000**power
fn get_power_of_thousand_pronunciation(power: i32) -> String {
	let mut string = String::new();	
	match power {
		1 => string.push_str("thousand"),
		2 => string.push_str("million"),
		3 => string.push_str("billion"),
		4 => string.push_str("trillion"),
		5 => string.push_str("quadrillion"),
		6 => string.push_str("quintillion"),
		7 => string.push_str("sextillion"),
		_ => panic!("Power Error: power: {}", power),
	}
	string
}

fn get_one_digit_pronunciation(digit: i64) -> String {
	let mut string = String::new();
	match digit {
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
		_ => panic!("One Digit Error: {}", digit),
	}
	string
}

// Helper for two digit pronunciation
// specific cases for numbers
// 11 through 19, which don't conform to
// normal naming conventions.
fn get_specific_case_pronunciation(number: i64) -> String {
	let mut string = String::new();	
	match number {
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
		_ => panic!("Tens Error: 10 through 19: {}", number),
	}
	string
}

fn get_multiple_of_ten_pronunciation(multiple: i64) -> String {
	let mut string = String::new();	
	match multiple {
		2 => string.push_str("twenty"),
		3 => string.push_str("thirty"),
		4 => string.push_str("fourty"),
		5 => string.push_str("fifty"),
		6 => string.push_str("sixty"),
		7 => string.push_str("seventy"),
		8 => string.push_str("eighty"),
		9 => string.push_str("ninety"),
		_ => panic!("Multiple of Ten Error: {}", multiple),
	}
	string
}
