

// external crates 
//  
fn main() {




    // fn main() {
    //                        // ---------+
    // }
    
    {
                let r;                // ---------+-- 'a
                                      //          |
                {                     //          |    'b<'a
                    let x = 5;        // -+-- 'b  |
                    r = &x;           //  |       |
                } //x dead->                    // -+       |
                                      //          |
                println!("r: {}", r); //          |
            }  //   r-> dangling / invalid   
           // x  -----------------------
            
    // {
    //     let x:&i32; ////--------------------------------
    //     let r = 5 ;     // r **********************************
    //      x=&r ;     
    //      println!("{}",x);         
    // } // r droped //******************************** */  ---------------------

     


   // scope -> lifetime    // 
   // 
    
let a = String::from("stringone");   /////------------------------- 'a   *****************



    let b = "stringtwo2";


  //

    let res = longest(a.as_str(), b);//--------   'a  

    println!("{}",res);

                                          //-------------- ///


         /////////////////////************* */
         


  // if this can turn out as a dangle or not?



    
}      //-------------------------------------------------------------*************
//  
// /ownerhship
//-> borrow checker -> & -> 
// -> scopes 
// compares the scopes of variables 
// rust -> lifetime 
// 
// -> annoatte the lifetimes explicitly 
// 
// ownership and lifetimes
/*
   // 'a ->   a  -> 'name 
      &i32 -> &'a i32 //reference with explicit lifetime -> 
      &mut i32 -> &'a mut i32 // mutable reference with explicit lifetime

   // <T>    -> <'a>

*/

// 'a  lifetime is in the scope of longest 

// ->  <'a> -> 
// a &b == 'a
//  

/*
   
your function is not returning any kind of ref


there is exactly one reference as input in your function
fn longest(a:&str) -> &str {
    a
}
  


impl types 
&self / &mut self -> methods 
*/

//why -> 
// specifying  liftimes explicitly does not change the liftime of variable

fn add(a:&i32) ->&i32 {
    a
}
/*
  
 fn taking multiple reference and returing ref
 
*/
fn longest<'a>(a:&'a str,b:&'a str)->&'a str {
    if a.len()>b.len() {
        a
    }else {
        b
    }
}

