use std::io;

fn get_numbers() -> (isize, isize) {

	let mut input = String::new();
	println!("Introduce primer numero");
	io::stdin().read_line(&mut input).expect("Error reading input");
	let n1 = input.trim().parse().expect("Error parsing number");
	println!("Introduce segundo numero");
	input.clear();
	io::stdin().read_line(&mut input).expect("Error reading input");
	let n2 = input.trim().parse().expect("Error parsing number");
	return (n1, n2);
}

fn add(x: isize, y: isize) -> (isize) {
	x + y
}
fn add_mul(x: isize, y: isize) -> (isize, isize) {
	(x + y, x * y)
}

fn calc(x: isize, y: isize, f: &dyn Fn(isize, isize) -> isize) -> isize {
	f(x, y)
}

fn help() {
	let menu = "The command to execute are:
	'bai' to exit the program
	'add' to add two numbers
	'add_mul' add and multiply two numbers
	'calc_add' add two number with function as argument
	'anonymous_mul' multiply two number with anonymous function
	'help' to show this menu";
	println!("{}", menu);
}
fn main() {
	println!("Simple Shell");
	println!("----------------");

	loop {
		print!("-> ");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let mut parts = input.trim().split_whitespace();
		let command = parts.next().unwrap();

		match command {
			"add" => {
				let (num1, num2) = get_numbers();
				let result = add(num1, num2);
				println!("Result: {}", result);
			},
			"add_mul" => {
				let (num1, num2) = get_numbers();
				let (res1, res2) = add_mul(num1, num2);
				println!("Result1: {}", res1);
				println!("Result2: {}", res2);
			},
			"calc_add" => {
				let (num1, num2) = get_numbers();
				let result = calc(num1, num2, &add);
				println!("Result: {}", result);
			},
			"anonymous_mul" => {
				let (num1, num2) = get_numbers();
				let result = (|x: isize, y: isize| {
					x * y
				})(num1, num2);
				println!("Result: {}", result);
			}
			"bai" => {
				println!("bye bye");
				return;
			},
			"help" => help(),
			_ => {
				println!("Wrong command");
				help();
			},
		}
	}
}
