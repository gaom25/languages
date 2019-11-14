// struct son los atributos de la clase
// trait es como la interface, donde se definen los metodos
// impl es la implementacion de los metodos

//As you can see methods take a special first parameter, the type itself. It can be either self, &self, or &mut self . self if it’s a value on the stack(taking ownership), &self if it’s a reference, and &mut self if it’s a mutable reference.


trait GetSound {
	fn get_sound(&self) -> String;
}

#[derive(Debug)]
struct Cat {
	name: String,
	sound: String,
}

impl GetSound for Cat {
	fn get_sound(&self) -> String {
		self.sound.clone()
	}
}

impl Cat {
	fn new(name: String, sound: String) -> Cat {
		println!("called method new cat");
		Cat {
			name: name,
			sound: sound,
		}
	}
}


#[derive(Debug)]
struct Bell {
	name: String,
	sound: String,
}

impl GetSound for Bell {
	fn get_sound(&self) -> String {
		self.sound.clone()
	}
}

impl Bell {
	fn new(name: String, sound: String) -> Bell {
		println!("called method new bell");
		Bell {
			name: name,
			sound: sound,
		}
	}
}

fn make_sound<T: GetSound>(t: &T) {//T should be implemeted from GetSound trait
	println!("{}!", t.get_sound());
}

fn main(){

	let kitty = Cat::new("Misifus".to_string(), "Meow".to_string());
	let the_bell = Bell {name: "big ben".to_string(), sound: "ding dong".to_string()};

	println!("kitty {:?}", kitty);
	make_sound(&kitty);
	println!("bell {:?}", the_bell);
	make_sound(&the_bell);
	
}