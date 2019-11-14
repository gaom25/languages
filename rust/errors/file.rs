use std::env;


fn function_with_error() -> Result<u64, String> {
    //if error happens
    if true {
        return Err("some message".to_string());
    }

    // else, return valid output
    Ok(255)
}

fn complex_function() -> Result<u64, String> {
    let x = function_with_error()?; // if Err, returns immidiately; if Ok(255), set x to 255

    // some other code, ex
    println!("{}", x); // 255 ; if you change line 20 `true` to `false`

    Ok(0)
}

fn get_full_name(fname: &str, lname: &str, mname: Option<&str>) -> String { // middle name can be empty
  match mname {
    Some(n) => format!("{} {} {}", fname, n, lname),
    None => format!("{} {}", fname, lname),
  }
}
fn main () {
	println!("Option enum");
	println!("{}", get_full_name("Galileo", "Galilei", None));
	println!("{}", get_full_name("Leonardo", "Vinci", Some("Da")));

	println!("Result enum");
	let key = "HOME";
    match env::var(key) {
        Ok(v) => println!("{}", v), // This prints "/root", if you run this in Rust playground
        Err(e) => println!("{}", e), // This prints "environment variable not found", if you give a nonexistent environment variable
    }

    println!("Unwrap");
    let x = function_with_error().unwrap();

    println!("{}", x);

    println!("unwrap deafult");
    let v = 8;
    let v_default = 0;

    let s_v: Option<i8> = Some(8);
    let n: Option<i8> = None;

    assert_eq!(s_v.unwrap_or_default(), v);       // Some(v) unwrap_or_default = v
    assert_eq!(n.unwrap_or_default(), v_default); // None unwrap_or_default = default value of v

    let o_v: Result<i8, &str> = Ok(8);
    let e: Result<i8, &str> = Err("error");

    assert_eq!(o_v.unwrap_or_default(), v);       // Ok(v) unwrap_or_default = v
    assert_eq!(e.unwrap_or_default(), v_default);

    println!("unwrap else");
    let v1 = 8;
    let v2 = 16;

    let s_v1 = Some(8);
    let n = None;
    let fn_v2_for_option = || 16;

    assert_eq!(s_v1.unwrap_or_else(fn_v2_for_option), v1); // Some(v1) unwrap_or_else fn_v2 = v1
    assert_eq!(n.unwrap_or_else(fn_v2_for_option), v2);    // None unwrap_or_else fn_v2 = v2

    let o_v1: Result<i8, &str> = Ok(8);
    let e: Result<i8, &str> = Err("error");
    let fn_v2_for_result = |_| 16;

    assert_eq!(o_v1.unwrap_or_else(fn_v2_for_result), v1); // Ok(v1) unwrap_or_else fn_v2 = v1
    assert_eq!(e.unwrap_or_else(fn_v2_for_result), v2);    // Err unwrap_or_else fn_v2 = v2

    println!("the ? operator");
    if complex_function().is_err() {
        println!("Can not calculate X!");
    }
	
}