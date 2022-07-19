use std::hash::Hash;
use std::io;

use std::collections::HashMap;

mod Dex;
use Dex::{Balances,Token,Pool}; 


fn main() {
    
    println!("Please enter your address");

   let mut address = String::new(); //buffer -> address of user 
    io::stdin().read_line(&mut address);



    let tokenA = Balances { token:Token::SOL, balance:10000.0 };
    let tokenB = Balances { token:Token::DOT, balance:10000.0 };
   

    let mut  my_pool = Dex::Pool::create_pool(tokenA, tokenB);
     
    let mut balances_of_user = HashMap::new(); // wallet 

     balances_of_user .insert(Dex::Token::USDT,1000.0); 


      let mut users_wallet = Dex::Wallet::create_wallet(address,  balances_of_user);
    // wallet

       
      println!("your wallet created ");
      

loop{
    println!("1-check your balance"); 
    println!("2- see the market"); 
    println!("3- buying token");
    println!("4- sell token");
    println!("5- swap  token");
    let mut  ch = String::new();
    io::stdin().read_line(&mut ch); 

    let choice:i32 = ch.trim().parse().unwrap();
      
    match choice {
         1 => {
          users_wallet.check_my_balance();
         },
         2=>{
            
          Token::show_current_market();
           
         },
         3=>{
             println!("which token you want to buy "); 
               Token::show_current_market(); 
             let mut token_name = String::new();
             io::stdin().read_line(&mut token_name);

             let buying_token = Token::return_token(token_name.trim());

             println!("enter the amount  "); 

             let mut token_amount = String::new();
             io::stdin().read_line(&mut token_amount);


             let parsed_token_amount:f64 = token_amount.trim().parse().unwrap();

             Token::buy_token(buying_token, parsed_token_amount, &mut users_wallet.balances)


         },
         4=> {
          println!("which token you want to sell");

          users_wallet.check_my_balance();
          let mut token_name = String::new();
          io::stdin().read_line(&mut token_name);
          let selling_token = Token::return_token(token_name.trim());
          let selling_token2 = Token::return_token(token_name.trim());
          println!("enter the amount  "); 

          let mut token_amount = String::new();
          io::stdin().read_line(&mut token_amount);


          let parsed_token_amount:f64 = token_amount.trim().parse().unwrap();


          Token::sell_token(selling_token, selling_token2,parsed_token_amount, &mut users_wallet.balances)

         },

         5 =>{
            
            println!("enter TOkenA amount ");
            let mut token_a_amount = String::new();
            io::stdin().read_line(&mut token_a_amount);
             
            let parsed_token_a_amount:f64 = token_a_amount.trim().parse().unwrap();
             Pool::swap_a_to_b(&my_pool, parsed_token_a_amount,&mut users_wallet.balances )  ;
            
         },
         _=>{
            println!("invalid option");
         }

         
    };


}

}  







