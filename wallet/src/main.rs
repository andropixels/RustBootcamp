use std::fmt::Debug;

use step4::Trait;
//4 steps
mod step1; 
mod step2;
mod step3;
mod step4;

fn main() {
  // u32
 let r = Run{
    a:3
 }; 

 


}

trait SomeTrait{
    type a ; 
    fn foo(b:Self::a) -> Self;
        
    
}

struct Run {
    a:i32
}

impl SomeTrait for Run {
    type a = i32;
    fn foo(b:Self::a) -> Self {
        Self { a: b }
    }
}



// testing 


// #[cfg(test)]
// mod test{
   
// }

#[test] 
fn test_step1() {
let mut balances = step1::Balances::new() ; 
let user_id_1 = 1;
let user_id_2 = 2;

balances.set_balance(user_id_1, 100);
balances.set_balance(user_id_2, 200);

// test if set balance is working or not

assert!(balances.show_balance(user_id_1)==100);
assert!(balances.show_balance(user_id_2)==200);

// if transfer is working or not 
// is_some / is_none bool 
// is_ok is_err bol 

assert!(balances.transfer(user_id_1, user_id_2, 50).is_ok()); 

assert!(balances.show_balance(user_id_1)==50);
assert!(balances.show_balance(user_id_2)==250);





}

#[test] 
fn test_step2() {
let mut balances = step2::Balances::new() ; 
let user_id_1 = 1;
let user_id_2 = 2;

balances.set_balance(user_id_1, 100);
balances.set_balance(user_id_2, 200);

// test if set balance is working or not

assert!(balances.show_balance(user_id_1)==100);
assert!(balances.show_balance(user_id_2)==200);

// if transfer is working or not 
// is_some / is_none bool 
// is_ok is_err bol 

assert!(balances.transfer(user_id_1, user_id_2, 50).is_ok()); 

assert!(balances.show_balance(user_id_1)==50);
assert!(balances.show_balance(user_id_2)==250);





}


#[test]
fn test_step3() {
    let mut balances = step3::Balances::new() ; 
    let user_id_1 = 1;
    let user_id_2 = 2;
    
    balances.set_balance(user_id_1, 100);
    balances.set_balance(user_id_2, 200);
    
    // test if set balance is working or not
    
    assert!(balances.show_balance(user_id_1)==100);
    assert!(balances.show_balance(user_id_2)==200);
    
    // if transfer is working or not 
    // is_some / is_none bool 
    // is_ok is_err bol 
    
    assert!(balances.transfer(user_id_1, user_id_2, 50).is_ok()); 
    
    assert!(balances.show_balance(user_id_1)==50);
    assert!(balances.show_balance(user_id_2)==250);
    
    
    
    
    
    }
    

#[test] 
fn test_step4() {

    // pub struct Balances<T:Trait> {
//     wallet:HashMap<T::AccountID,T::Balance> 
// }

// T:Trait 
// <T as Trait>::Balance
 
struct T ; 

impl step4::Trait for T {
     type AccountID = u32;
     type Balance = u64;
}

let user_id_1: <T as step4::Trait>::AccountID = 1 ; 
let user_id_2:<T as step4::Trait>::AccountID = 2;

let mut balances = step4::Balances::<T>::new() ; 


balances.set_balance(user_id_1, 100);
balances.set_balance(user_id_2, 200);

// test if set balance is working or not

assert!(balances.show_balance(user_id_1)==100);
assert!(balances.show_balance(user_id_2)==200);


assert!(balances.transfer(user_id_1, user_id_2, 50).is_ok()); 

assert!(balances.show_balance(user_id_1)==50);
assert!(balances.show_balance(user_id_2)==250);




}



