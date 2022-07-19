
use std::{ fmt::Debug};


fn main() {

    // print_str("rahul");
    // prints_int(4);
    // prints_float(9.0);

                            //i32
                            let ps = Printing{
                                field:3

                            };
                            
    prints_everything::<i32>(9); 
    prints_everything::<f32>(3.0);
    prints_everything::<&str>("");
    prints_everything::<Printing>(ps);  // custom data type 

 // i need everything in check at compile time 


}

#[derive(Debug)]
struct Printing{
    field:i32,
}

fn prints_int(a:i32) {
    println!("{}", a);
}

fn prints_float(a:f32) {
    println!("{}", a);
}

fn print_str(a:&str){
    println!("{}", a);

}


// with generics only one fn  <>
/// provding a placeholder for your types 

// T-> i32
//T-> String
//T -> custom type 
// T -> f32
// T -> 
//<T:Debug>
fn prints_everything<T:Debug>(a:T) {

    println!("{:?}", a);

}
