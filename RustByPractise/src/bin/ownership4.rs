// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    let num = 9; 
    // const and static 

    match num {
        1 => println!("it is one "),
         2 => println!("it is 2 ") ,
         9 => println!("it is 9 "),
         _ => println!("not 1 , 2 or 9 ")

    }
let s = String::from(""); // owned string/ heap allocted atring 
//  s.push_str(""); //growableb -> heap 

 let s1 := &s ; // referncee -> borrows 
 // String -> &str 



let borrowed_string = "name";

  let owned_string = borrowed_string.to_string();
  let owned_string = borrowed_string.to_owned();




 //statck -> varaibles whose value is known at compile time 
 // heap -> 

  
// growable string
let d =" ";// &str -> borrowed string, stack allocated and it is



// not growable 
// mutable and growable 




    println!("{}", s);
    print_str(String::from("_"))
    // ownership 
    

}

fn print_str(s: String)  {
    println!("{}",s)
}


