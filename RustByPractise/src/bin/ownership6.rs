// Mutability can be changed when ownership is transferred.

// fn main() {
//     let s = String::from("hello, ");
//     let mut  s1 =  s;
// }



// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }



fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


//   fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    let mut s = String::from("hello");

        println!("{}",s);
        let r1 = &mut s;
     // r1 goes out of scope here, so we can make a new reference with no problems.

     r1.push_str("rahul");
     

     println!("{}",r1);
     println!("{}",s);

    let r2 = &mut s;
    r2.push_str("i am r2");
    println!("{}",r2);
    println!("{}",s);
    
}






