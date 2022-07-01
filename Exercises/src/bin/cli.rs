/*
build a cli app which 
which accepts menu items 
and prints yes if that is present in your 
menu

*/

fn main() {


    let mut  s1 = String::from("hello");
     
    // let len = calculate_length(&s1); // trabnsfer ownership of s into func

    // println!("{}", len);
    

    // println!("{}",r1);



    // println!("{} ",r2);


     let r1 =&s1;
     let r2=&s1;
     let r3=&s1;
     let r4=&s1;


      {  
        let r6 = &mut s1;
      } // r6 -> delted

      let r7 = &mut s1; 

    //  r1.push_str("string");
     println!("{}",s1);

    //  let r2 = &mut s1 ;//-> hellostring// r1 -> invalid 

    //  r2.push_str("string2");
    //  println!("{}",s1);

    //   println!("{} ",r2);

      
let  mut s2 = String::from("rusting");

let m1= &s2; //-> rusting // begins 
let m2 =&s2; // -> rusting // begin

println!("{} {}" , m1,m2);  //  m1 and m3 scope ends here 

println!("{} {}",m1,m2);

let m3 = &mut s2; 




println!("{}",m3);



// RUST COMPILER CANT TOLERATE INVALID REFERNECE 









    // //ownership 
    //  let res = dangle() ; 

    //  let my_string =String::from("rust");

    //  let refr = &my_string;
    //  let ref refr2 = my_string;


     
      for i in 0..=12 {
        println!("{}",i);

      }

    let a = 0 ; 


    while a!=5 {
        println!("{}",a);
        a = a+1;
    }


    let b =5 ;
    loop {
        
        if b==0 {
            break;
        }
        println!("rust");

        b= b-1;

    }



    let my_vec = Vec::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    my_vec.pop();  // 3 
    my_vec.len();


    println!("{:?}",my_vec);


    let my_vec1 = vec![1,2,3]; 
    my_vec1.push(4);








    


}

fn dangle() -> &String {

     let s = String::from("");
    &s   //-> s
}// s is deleted 

//RUST COMPILER CAN NOT TOLERATE INVALID REFERENCE 







fn change(s:&mut String) {

    s.push_str("string");
    println!("{}",s);
}


fn calculate_length(s:&String) ->usize  {


    s.len()
}


