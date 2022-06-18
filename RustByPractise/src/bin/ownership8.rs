
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t);
 }

 /*
  Hints: This is a case of partial move
 */

 