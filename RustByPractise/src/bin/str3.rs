
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}


/*
Hint : only String and &str can be concated 
*/

