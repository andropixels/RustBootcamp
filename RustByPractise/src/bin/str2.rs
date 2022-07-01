
// Fix all errors without adding newline
fn main() {

    let mut  s =  String::from("hello");
    s.push(',');
    s.push_str(" world");

    //&str -> String
    let name = "name" ; // borrowed string 
    let owned_name = name.to_string();// owned string 
    let owned_name2 = name.to_owned();
    


    s += "!";

    // println!("{}", s);


//     let mut s: String = String::from("hello");
// s.push(',');
// s.push_str(" world");
// s += "!";

// println!("{}", s);

}






/*
 Hint:  read the difference between push and push_str
*/




