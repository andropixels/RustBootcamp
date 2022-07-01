
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // let s3 = s1 + s2; // feature
    //  -> ownership 
    // 

    let s4 ="";
    let s5 = "";
    // let s6 = s4+s6;
// result -> String

    // let s7 = s4 + s1; 
    // s4 = "name" s1 ="lastname"
    // s7 = namelastname 
    //  &str + String 

    // String + &str 
    //

    // let x = 9 ; 
    // the owner of 9 is x 

let mut y =9; 
    {
        let x  = 10 ; //
         

    } // 
    
    //  -> deletes the x 
    // fn main() {
    //     let mut x = 10;
    //     println!("{}",x);
    //     x = 7;
    //     println!("{}",x);
    //     }
//    00000
//     7
//
    {
        let mut s = String::from("");


    }// s will be dropped here 


    let x = 5 ; // 8042785

    let y = x ; //87348745 -> shallow copy 



    println!("{}", x); 
    // ^^ value borrowed here after move


     let s = String::from("hello"); //  -> growable heap 
     let s1 =""; //-> &str -> stack not growable 
     //-> s is growable -> heap(growable ) 
      // owned string // heap
    // s is the owner of hello 

    {

        let s = String::from(""); // begin

    }// end here and s will be deleted by compiler 



let s4 = String::from("hello");
//stack -> ptr, len and capcity 
// heap -> "hello"
let s5 = s4 ; // ownership ("hello") will be trabsfered tpo s5 
// rust compiler delets s4 
// ptr 
//lrn
//cap 
//hello 
// s4 wiill be deleted
// move 

let s5 = &s4 ; // refernce ownership is not trabsfereed 


println!("{}", s4); 

let s5 = s4.clone(); // deep copy 
println!("{}", s4); 








    let x =0;  // stack
    let y = x ;   // x's value to y -> shallow copy  // stack 
    // println!("{}", x);
    let s2 = String::from("rahul"); // heap
    let s3 = s2 ;  //

    // println!("{}", s2);



















    {

        let y =0 ; //  y's begins //
        // during compile time 
        // primary -> int, fload , char 

    } // end at this point 
    println!("{}", y); 

    // x is the owner of 0 ; 

     let s1  = "name";// stack 

     // 

     let s1 = s.clone() ;  // deep  copy 
    // s->hello &  s1 -> hello -> shallow copy + deleting s (move)



     // move -> ownership transfers 




    //  println!("{}",s);




    // copying

    // println!(s);


    // println!("{}", x);


    //&str -> ungrowable
    // resul -> String -> growable 
    //  result -> String 



    // assert_eq!(s3,"hello,world!");
    // println!("{}",s1);

     let mut stri = String::from("hello ");  //here
//sx will be deleted 
    // let s2 = &s; // invalid reference -> dangling reference  
    // let s3 = &s ; 
    // let s4 = &s ; 

    let s5 = &mut stri;  // one and only one 
    let s6 = &mut stri ; 

    println!("{}",s6);


    // take_ownership( s);

    // println!("{}",s); 
}// 


/*
Hint : only String and &str can be concated 
*/

// //s1->  s -> "hello "  
// fn take_ownership(s1: String){


//     println!("{}",s1.push_str("name")); 
// }

