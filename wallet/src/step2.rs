
use std::collections::HashMap; 
type AccountID = u32 ;   
type Balance = u32 ; 

pub struct Balances {
    wallet:HashMap<AccountID,Balance> 
}




impl Balances {

    // set_balance
    // transfer
    // show_balance

    // new -> 
    pub fn new() -> Self {
        Self { wallet: HashMap::new() }
    }

  pub fn set_balance(&mut self, user_id:AccountID,amount:Balance) {
        // 
        self.wallet.insert(user_id, amount); 
    }

   pub  fn transfer(&mut self,from:AccountID,to:AccountID,amount:Balance) -> Result<(),String> {
         // from to amount 
         let from_bal = self.wallet.get(&from).ok_or("can  not fetch out balance ")?;
         // from_bal ? 
         // OK(4) => 4 
         // ? -> quetion mark 
         // match 
         //Ok(()) => match / unwrap / ? / restriction => 
         // ?-> Result
         let to_bal = self.wallet.get(&to).unwrap_or(&0);
         // None => to 
         //Some(data)_or(7)

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
    
  pub  fn show_balance(&self,user:AccountID) -> u32 {
        *self.wallet.get(&user).unwrap_or(&0)
    }
   

}



