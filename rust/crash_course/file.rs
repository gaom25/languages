fn foo1(mut i: isize) -> isize {
    i += 1;
    i
}

fn foo2<'a>(mut i: &'a mut isize, j: &'a mut isize) {
    *i *= 10;
    i = j;
    *i *= 10;
}

fn test1() {
    let i: isize = 1;
    let j: isize = foo1(i);
    println!("{}", j);
}

fn test2() {
	let mut i: isize = 1;
    let mut j: isize = 2;
    foo2(&mut i, &mut j);
    println!("{} {}", i, j);
}
/*fn test3() {
	let hello1 = String::from("Hello, ");
    let hello2 = String::from(", hello!");
    let name = "Alice";
    println!("{}", hello1 + name);
    println!("{}", name + hello2);
}*/
fn test4() {
	for arg in std::env::args().skip(1) {
        match arg.as_ref() {
        	"hi" => println!("Hello there!"),
        	"bye" => println!("OK, goodbye!"),
        	_ => println!("Sorry, I don't know what {} means", arg),
    	}
    }
}
/*fn test5() {
	let hello = String::from("Hello, ");
    let greet = |name| hello + name;
    println!("{}", greet("Alice"));
    println!("{}", greet("Bob"));
}*/

fn main() {
	test1();
	test2();
	test4();
	//test5();
}