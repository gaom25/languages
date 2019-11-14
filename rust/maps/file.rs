use std::collections::HashMap;

fn main () {
	let mut scores = HashMap::new();
	let team_blue = String::from("Blue");
	let team_yellow = String::from("Yellow");
	let team_red = String::from("Red");

	scores.insert(&team_blue, 10);
	scores.insert(&team_yellow, 50);
	println!("Map {:?}", scores);

	let score = scores.get(&team_blue);

	println!("score of blue {:?}", score);

	scores.entry(&team_blue).or_insert(200);
	scores.entry(&team_red).or_insert(20);

	println!("Score updated? {:?}", scores);

	let text = "hello world wonderful world";
	let mut map = HashMap::new();

	for word in text.split_whitespace() {
	    let count = map.entry(word).or_insert(0);
	    *count += 1;
	}
	println!("Map update base on old value");
	println!("text [{}]", text);
	println!("{:?}", map);
}