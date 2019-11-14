/*
fn main() {
  let mut a = vec![1, 2, 3];
  let b = &mut a;  //  &mut borrow of a starts here
  // some code
  
  println!("{:?}", a); // trying to access a as a shared borrow, so giving error
} 
*/

fn main() {
  let mut a = vec![1, 2, 3];
  {
    let b = &mut a;  //  &mut borrow of a starts here
    b[0] = 4;
  }                  //  &mut borrow of a ends here
  
  println!("{:?}", a); // allow to borrow a as a shared borrow
}



// life times 
//https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language
/*
// no inputs, return a reference
fn function<'a>() -> &'a str {} 

// single input
fn function<'a>(x: &'a str) {}

// single input and output, both has same lifetime
// output should live at least as long as input exists
fn function<'a>(x: &'a str) -> &'a str {}

// multiple inputs, only one input and the output share same lifetime
// output should live at least as long as y exists
fn function<'a>(x: i32, y: &'a str) -> &'a str {}

// multiple inputs. both inputs and the output share same lifetime
// output should live at least as long as x and y exist
fn function<'a>(x: &'a str, y: &'a str) -> &'a str {}

// multiple inputs. inputs can have diffent lifetimes 🔎
// output should live at least as long as x exists
fn function<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {}

// single element
// data of x should live at least as long as Struct exists
struct Struct<'a> { 
    x: &'a str 
}

// multiple elements
// data of x and y should live at least as long as Struct exists
struct Struct<'a> { 
    x: &'a str,
    y: &'a str 
}


// variant with single element
// data of the variant should live at least as long as Enum exists
enum Enum<'a> { 
    Variant(&'a Type)
}


fn greeting<'a>() -> &'a str {
  "Hi!"
}


fn fullname<'a>(fname: &'a str, lname: &'a str) -> String {
  format!("{} {}", fname, lname)
}


struct Person<'a> { 
    fname: &'a str,
    lname: &'a str
}
  impl<'a> Person<'a> {
      fn new(fname: &'a str, lname: &'a str) -> Person<'a> { //no need to specify <'a> after new; impl already has it
          Person {
              fname : fname,
              lname : lname
          }
      }

      fn fullname(&self) -> String {
          format!("{} {}", self.fname , self.lname)
      }
  }

fn main() {
    let player = Person::new("Serena", "Williams");
    let player_fullname = player.fullname();
    
    println!("Player: {}", player_fullname);
}
*/