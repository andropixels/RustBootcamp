// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32> // locker number


struct Lockers {
    name_student:String, 
    locker_assigned:Option<i32>
}
 

enum Nested{
    Op(Option<String>)
}


impl Nested {
    fn new(s:&str) -> Self {
        if s.len() >=3{
            Self::Op(None)

        }else {
            Self::Op(Some(s.to_owned()))
        }
       
    }
}



fn main() {


    // let l1 = Lockers{
              

    //     name_student:"Rahul".to_string(),
    //     locker_assigned:Some(1)
    // };


    // match l1.locker_assigned {
    //     Some(data) =>println!("name :{:?} locker number:{}", l1.name_student,data) ,
    //     None => println!("no locker assigner" )
    // }



    // let l2 = Lockers{
              

    //     name_student:"sanget".to_string(),
    //     locker_assigned:None
    // };

    // match l2.locker_assigned {
    //     Some(data) =>println!("name :{:?} locker number:{}", l2.name_student,data) ,
    //     None => println!("no locker assigner" )
    // }


    let nest = Nested::new("go");

     match nest {
        Nested::Op(data) => {
            match data{
                Some(data) => println!("the data is {}", data),
                None => println!("lenghth is too short")
            }
        }
     }

    
}
