// wallet 
// ?? 

// hashmap
// user_id => balance

use std::collections::HashMap;
// 
pub struct Balances {
    wallet:HashMap<u32,u32> 
}


impl Balances {

    // set_balance
    // transfer
    // show_balance

    // new -> 
    pub fn new() -> Self {
        Self { wallet: HashMap::new() }
    }

  pub fn set_balance(&mut self, user_id:u32,amount:u32) {
        // 
        self.wallet.insert(user_id, amount); 
    }

   pub  fn transfer(&mut self,from:u32,to:u32,amount:u32) -> Result<(),String> {
         // from to amount 
         let from_bal = self.wallet.get(&from).ok_or("can  not fetch out balance ")?;
         // from_bal ? 
         // OK(4) => 4 
         // ? -> quetion mark 
         // match 
         //Ok(()) => match / unwrap / ? / restriction => 
         // ?-> Result
         let to_bal = self.wallet.get(&to).unwrap_or(&0);
         // unwarp_or()Some 
         // none => 

         // 
         let new_from_bal = from_bal.checked_sub(amount).ok_or("not enough balance")?;
         // checked_add = > u32 underflow / overflow 
         // 
         let new_to_bal = to_bal.checked_add(amount).ok_or("overflow")?;

         // wallet ledger
         self.wallet.insert(from, new_from_bal);
         self.wallet.insert(to, new_to_bal);

         Ok(())
    }
    
  pub  fn show_balance(&self,user:u32) -> u32 {
        *self.wallet.get(&user).unwrap_or(&0)
    }
   

}

