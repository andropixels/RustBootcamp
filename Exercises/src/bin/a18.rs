// Topic: Result
//
// Requirements:
// * Determine if a customer is able to enter in  a theater 
// * Restricted entrns  require that the age of the customer
//   is at least 21 
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted entry
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


// struct customer {
//     age:i32
// }

//age>=21 
//() -> unit
fn entry(age:i32) -> Result<(),String> {

    if age>=21{

        Ok(())
    }else {
        Err("not allowed".to_owned())
    }
}




fn main() {

    let entry = entry(28);
    match entry {
        Ok(d)=> println!("{:?}",d),
        Err(e) => println!("{}",e)
    }
}
