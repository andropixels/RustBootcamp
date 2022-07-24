// trait + type + trait bounds 

use std::collections::HashMap;

use std::hash::Hash;
use num::traits::{Zero,CheckedAdd,CheckedSub};
pub trait Trait {
type AccountID:Hash+Eq ;   
type Balance:Zero+CheckedAdd+CheckedSub+Copy; 
}


pub struct Balances<T:Trait> {
    wallet:HashMap<T::AccountID,T::Balance> 
}


// T:Trait 
// impl<>
impl<T:Trait> Balances<T> {

    pub fn new() -> Self {
        Self { wallet:HashMap::new() }
    }

    // T:Trait
    /*
       Trait{
        type AccountId
        type Balance
       }
    */
     pub fn set_balance(&mut self, user_id:T::AccountID,amount:T::Balance) {
                self.wallet.insert(user_id, amount);
     }

     pub fn transfer(&mut self , from:T::AccountID,to:T::AccountID,amount:T::Balance) -> Result<(),String> {
                                                                                            // &str -> String                    
    let from_bal = self.wallet.get(&from).ok_or("can not fetch from's blance")?;
    // &<T as Trait>::Balance  -> T:Trait == <T as Trait>::AccountId
    let zero = T::Balance::zero();
//      <T as Trait>::Balance
    let to_bal  = self.wallet.get(&to).unwrap_or(&zero);

    let new_form_bal = from_bal.checked_sub(&amount).ok_or("can not add")?; 
    let new_to_bal = to_bal.checked_add(&amount).ok_or("overflow")?;

    self.wallet.insert(from, new_form_bal);
    self.wallet.insert(to, new_to_bal);

    Ok(())

     }

     pub fn show_balance(&self,user_id:T::AccountID) -> T::Balance {
        *self.wallet.get(&user_id).unwrap_or(&T::Balance::zero())
        
    }
}

// 