// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter




use std::collections::HashMap;



#[derive(Clone,Copy,PartialEq,PartialOrd)]
struct GroceryItem {
    quantity:i32,
    id:i32
}


fn display_quan(gi:GroceryItem) {

    println!("{:?}", gi.quantity);

}


fn display_id(gi:GroceryItem) {

    println!("{:?}", gi.id);


}





fn main() {

    let gi = GroceryItem{
        quantity:7,
        id:9
    };

    let gi2 = GroceryItem{
        quantity:8,
        id:0,
    };

    if gi == gi2  {


        
    }

    display_quan(gi);
    display_id(gi);


    let mut  my_map = HashMap::new();




     my_map.insert(1,My::No);

     my_map.insert(6,My::Yes);
     



     let val = my_map.get(&1).unwrap();

     assert_eq!(my_map.contains_key(&1), true);


    //  for val in my_map.values() {
    //     println!("{val}");
    // }

    // for key in my_map.keys() {
    //     println!("{key}");
    // }







    //   println!("{}",val+5.4);









}



#[derive(  PartialEq,Eq)]
enum My {

    No,
    Yes
}

