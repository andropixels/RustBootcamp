

/*

write a  generic function  having three parameters of generic ty&pe  and     return the greatest number among them 
use this function for i32 and f32 types

*/

// return type? 
// generic -> primitive 
// ref -> liftimes 
// greater_number is generic over A 
// custom types == primitive derive/ implemnt 
// generics == primitve / implementing traits 
// <T:Debug>  

// genric function 

use std::fmt::Debug;

use std::io; 

// <>
// 
fn greater_number<A:PartialOrd>(a:A,b:A,c:A) -> A {


         if a>b && a>c {
            return a;
         }else if b>a && b> c{
            return b ;
         }else {
            return c; 
         }

        //   let d = Drive::Direction("UP",40);

    



}

fn multiple_gen<A:Debug,B:Debug,C:Debug> (a:A,b:B,c:C) {


    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);
}




// custom types 
// == !=
// >= <=

// generic enum 


//Derive is genric over T
enum Drive<T>{

    Direction(String, T)
}




#[derive(PartialEq,PartialOrd)]
struct A {
    num:i32
}

#[derive(PartialEq, PartialOrd)]
struct B {
    i:i32
}

// multiple generics 
// SOmething is generic over A,B and C 
enum Something<A,B,C> {

      
      Data(A,B,C)
}


// Option and Result 

// Option -> enum which is generic over T and it has two varients 
// // Some and None
// enum Option<T> {
//   Some(T),
//   None

// }

// 

// None == null 
// null , nil
enum Option<T>{
    Some(T),
    None 
}

fn main() {

let mut  var1= Some("name"); 

let mut var2 = Some(());  // option of uint 

match var2 {
    Some(data) => println!("{:?}", data),
    _ => println!("panic")
}

// extract data 



match var1 {
    Some(data) => println!(" data over var1 {}" , data),
    None => println!("null value ")
}

   
    
    let res1 = greater_number::<char>('a','b','c');
    // let res2 = greater_number::<f32>(10.0,30.0,4.0);

    println!("{}", res1);
let d = Drive::<i32>::Direction("UP".to_string(),5);
let some_thing_i32_1 = Something::<i32,f32,String>::Data(3,5.0,"S".to_owned());
let some_thing_i32_2:Something<i32,f32,String> = Something::Data(4, 5.0, String::from(""));
match some_thing_i32_1 {

    Something::Data(data0, data1,data2 )=> println!("{} {} {}", data0,data1,data2)

}

       


    // let some_thing_i32_2 = Something::<i32>::Float(3);
    // let some_thing_i32_3 = Something::<i32>::Stringg(3);
    // let some_thing_f32_1= Something::<f32>::Num(3.0);
    // let some_thing_f32_2= Something::<f32>::Float(3.0);
    // let some_thing_f32_3= Something::<f32>::Stringg(3.0);


// let mut buffer = String::new();
// io::stdin().read_line(&mut buffer); 

// let parse_i32 = buffer.trim().parse(); 



let res = returns_option().unwrap();  


//  i32 
// // 
// match res {

//     Some(dta) =>
// }



  
      
}


fn returns_option() -> Option<i32>{

     let var1 = Some(4); 
     return var1 ;
}
// unwrap -> 

/*
      

     enum Ordinates{
        X(i32),
        Y(i32),
        Z(i32)
     }

     
 */