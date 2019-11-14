// https://stackoverflow.com/questions/46388386/what-exactly-does-derivedebug-mean-in-rust
// exists generics
#[derive(Debug)]
struct Employee {
	first_name: &'static str,
	last_name: &'static str,
	salary: isize,
}

//like named tuple 
struct Salary (i32, i32, i32);

enum Charge {
	Ceo,
	Leader{area: &'static str, employees: i32},
	Worker(String)

}

#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}
/*defined enum in rust like haskell first its maybe the second is result
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn get_id_by_title(title: &str) -> Option<&str> {
	return Some(title)
}

fn print_message(m: Charge) {
	match m {
		Charge::Ceo => println!("its the CEO"),
		Charge::Leader{area, employees} => println!("Leader: {} - {}", area, employees),
		Charge::Worker(msg) => println!("worker {}", msg),
	}
}

fn main() {

	println!("Struct");
	let mut pedro = Employee {
		first_name: "pedro",
		last_name: "perez",
		salary: 200
	};

	println!("{:?}", pedro);
	pedro.salary = 100;
	println!("{:?}", pedro);

	let pablo = Employee {first_name: "pablo", .. pedro};
	println!("{:?}", pablo);

	let Employee {first_name: name, last_name: last, salary: sal} = pablo;
	println!("desconstruct ({},{},{})", name, last, sal);

	println!("Tuple struct");
	let salary = Salary(10, 20, 30);
	let Salary(a, b, c) = salary;
	println!("salary destrcuture ({}, {}, {})", a, b, c);

	println!("Enums");
	let mut enum_example = Charge::Ceo;
	print_message(enum_example);

	enum_example = Charge::Leader{area: "dev", employees:12};
	print_message(enum_example);

	enum_example = Charge::Worker(String::from("gustavo"));
	print_message(enum_example);

	println!("Generics");
	let point_a = Point { x: 0, y: 0 };
	println!("point a {:?}", point_a);
  	let point_b = Point { x: 0.0, y: 0.0 };
  	println!("point b {:?}", point_b);

  	let title = "lolo";
  	println!("title {}", title);
  	match get_id_by_title(title) {
  		Some(i) => println!("asigned: {}", i),
  		None => println!("Not found :("),
  	}


}