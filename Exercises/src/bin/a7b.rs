//Topic:Working with enum
//
//

/* 
1-Define a Enum Phones with some brands of phones, 
2-define an enum Headsets with some brands  , 
3-define a struct Shop having two fields Phones and Headsets, 
4-create instance of Shop and print which phone and headsets you have picked 
*/


// #[derive(Debug)]
#[derive(Debug)]
enum Phones{

    Samesung,
    apple,
    vivo,

}
#[derive(Debug)]
enum Headsets {
    Sony,
    JBl,
    Boat
}

#[derive(Debug)]
struct Shop {
    phone:Phones,
    headset:Headsets
}




fn main(){

    let s1 = Shop{
        phone:Phones::Samesung,
        headset:Headsets::Boat,
    };


    println!("{:?}", s1);

}




