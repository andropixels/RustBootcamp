
// Fix all errors without adding newline
fn main() {
    let  s =  String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}

/*
 Hint:  read the difference between push and push_str
*/




