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

fn test_division(numerator: isize, denominator: isize) {
    match numerator.checked_div(denominator) {
        Some(result) => println!("{} / {} = {}", numerator, denominator, result),
        None => println!("{} / {} results in a division by zero", numerator, denominator)
    }
}

fn main () {
	let (num1, num2) = get_numbers();
	test_division(num1, num2);
	
}