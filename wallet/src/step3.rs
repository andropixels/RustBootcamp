
use std::collections::HashMap;
use std::hash::Hash;
use num::traits::{Zero,CheckedAdd,CheckedSub};

// balances structure is generic over 
// AccountId and Balances
pub struct Balances<AccountID,Balance> {
    wallet:HashMap<AccountID,Balance> 
}

// trait bound + 
impl<AccountID:Hash+Eq,Balance:Zero+CheckedAdd+CheckedSub+Copy+Clone> Balances<AccountID,Balance>{


    pub fn new() -> Self{
        Self { wallet: HashMap::new() }
    }
    pub fn set_balance(&mut self,user_id:AccountID,amount:Balance) {
        /**
         * generic structs impl
         * hashmap<key:AccountID val:Balance>
         * hashmap<u32,u32>
         * 
         */
        self.wallet.insert(user_id, amount);

    }

    pub fn transfer(&mut self, from:AccountID,to:AccountID,amount:Balance) -> Result<(),String>{
        let from_bal = self.wallet.get(&from).ok_or("can not fetch from's balance")?;
        // Zero trait 
        let zero = Balance::zero(); // Balance
        let to_bal = self.wallet.get(&to).unwrap_or(&zero);
        // Balance => 0 
        // T => 0 
        // Balance 
        // T == 0 
        // T ==T::One()
        // 
        // 
        let new_from_bal = from_bal.checked_sub(&amount).ok_or("not enough balance")?;

        let new_to_bal = to_bal.checked_add(&amount).ok_or("overflow")?;

        self.wallet.insert(from,new_from_bal);
        self.wallet.insert(to,new_to_bal);

        Ok(())
    }


    // pub  fn show_balance(&self,user:AccountID) -> u32 {
    //     *self.wallet.get(&user).unwrap_or(&0)
    // }
    pub fn show_balance(&self,user_id:AccountID) -> Balance {
        *self.wallet.get(&user_id).unwrap_or(&Balance::zero())
        
    }

}

// T -> 
