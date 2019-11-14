fn main() {

	let array = [1, 2, 3, 4, 5];
	println!("array {:?}", array);
	let tuple = (1, "a", true);
	println!("tuple {:?}", tuple);
	let slice = &array[0..2];
	println!("slice {:#?}", slice);
	let mut vector = vec![1,2,3,5,6];
	println!("vector {:?}", vector);
	vector.pop();
	vector.push(55);
	println!("vector {:?}", vector);


}
