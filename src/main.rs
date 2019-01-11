use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
	let number: i32 = args[1].parse().unwrap();

    println!("{}", digit_pronunciation(number));
}

fn digit_pronunciation(x: i32) -> String{
	let mut string = String::new();	
    match x {
        1 => string.push_str("one"),
        2 => string.push_str("two"),
        3 => string.push_str("three"),
        4 => string.push_str("four"),
        5 => string.push_str("five"),
        6 => string.push_str("six"),
        7 => string.push_str("seven"),
        8 => string.push_str("eight"),
        9 => string.push_str("nine"),
		_ => panic!("Digit Error"),
    }
	string
}
