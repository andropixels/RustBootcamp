use std::hash::Hash;
use std::io;

use std::collections::HashMap;
// listing tokens  -> 
#[derive(Debug,PartialEq,Hash,Eq,Clone, Copy)]
pub enum Token{

SOL,
DOT,
ETH, 
BTC, 
USDT
}



// buy_token(token:Token, amount:f64, balances:Vec<Balances> )



impl Token {

pub  fn show_current_market() {
// Self == TOken::
    println!("BTC: price:{}",Self::return_price(&Token::BTC) );
    println!("ETH: price:{}",Self::return_price(&Token::ETH) );
    println!("SOL: price:{}",Self::return_price(&Token::SOL) );
    println!("DOT: price:{}",Self::return_price(&Token::DOT) );
    println!("USDT: price:{}",Self::return_price(&Token::USDT) );
}




pub  fn return_token(input:&str) -> Self {

    match input {
        "sol"=> Token::SOL,
        "dot"=> Token::DOT,
        "btc"=> Token::BTC,
        "eth"=> Token::ETH,
        "usdt"=> Token::USDT,
        _=> Token::BTC
    }
}

fn return_price(&self) -> f64 {

    match self{
        Token::SOL=> 34.0,
        Token::BTC=> 30000.0,
        Token::ETH=> 1000.0,
        Token::DOT=> 8.0,
        Token::USDT=> 1.0,

    }

}



pub  fn buy_token(self, amount:f64, balances:&mut  HashMap<Token,f64>) {

      // 1000 usdt 
       // self 
       //1000   
       //usdt >= current price(self)*amount 
       // 10 *8 = 80 dot
       // 40 *34  = 1360 

       // get usdt of balacnce of user 
      let users_usdt_bal = balances.get(&Token::USDT).unwrap();
      // calculate the price 

      let price_of_token = Self::return_price(&self);
      let calculated_price_of_token = price_of_token*amount;

      if users_usdt_bal >= &calculated_price_of_token {
              
     /// update users wallet 
     /// 
         balances.insert(Token::USDT,users_usdt_bal-&calculated_price_of_token );

         if balances.contains_key(&self) {
            let prev_bal_of_token = balances.get(&self).unwrap();
            balances.insert(self, &amount+prev_bal_of_token);
         }else {
            balances.insert(self, amount);
         }
         
           println!("Transaction is succesfull !!! ");
        

            
      }else {

        println!("insuffiecuent balance");
        println!("Transaction declined ");
      }


        // for =>  usdt 
        // 

       

    }

 pub    fn sell_token(self,sell_token:Token,amount:f64,balances:&mut HashMap<Token,f64> ){

        // let copy_sell_token= &self;
               let bal_of_token = balances.get(&self).unwrap();
               if bal_of_token>=&amount {

                balances.insert(self, bal_of_token-amount);
                let prev_bal_of_usdt = balances.get(&Token::USDT).unwrap();
                let price_of_token = Self::return_price(&sell_token);
               let calculated_price_of_token = price_of_token*amount;
                let cal_usdt_bal = prev_bal_of_usdt+calculated_price_of_token;
                balances.insert(Token::USDT,cal_usdt_bal );

                        println!("TRansction is succeasfull ");
               }else {
                println!("invalid amount ");
                println!("transaction declined");
               }

    }


 

}


//liquidity pool 
//(tokenA, tokenB)
//SOL,USDT, balances
// (token:Token,balances:f64)
#[derive(Debug,Clone, Copy)]
pub struct Pool {
tokenA:Balances<Token>,
tokenB:Balances<Token>
}

impl Pool {

pub fn create_pool(tokenA:Balances<Token>,tokenB:Balances<Token>)-> Self{

    Self { tokenA, tokenB }

}

//direction 
pub  fn swap_a_to_b(&self,amout_of_a:f64,balances:&mut HashMap<Token,f64>) {

       let token_a= self.tokenA.token;// insert
       let token_b= self.tokenB.token;// return

// (A_total + A_in) * (B_total - B_out) = invariant
// (100 + 10) * (5,000 - B_out) = 500,000
// 5,000 - B_out = 500,000 / 110
// 5,000 - (500,000 / 110) = B_out
// B_out = 454.5454...
let users_bal_of_token_a = balances.get(&token_a).unwrap();

// user_wallet_token_A >= amount_of_swap
// tokena= 10
//10



if users_bal_of_token_a>=&amout_of_a {

let a_init = &self.tokenA.balance;   // pool's initial value of token a
let b_init= &self.tokenB.balance  ;// pool's initial value of token b
//x*y =k 
let invarient = a_init*b_init; 

// (A_total + A_in) * (B_total - B_out) = invariant
let a_cal = a_init+amout_of_a;


let div = invarient/a_cal;
let mut b_out = b_init-div ; // amount of b i am getting for AtoB swap

println!("Estimeated amount of {:?} is {:?}, do you want to procces this swap?",token_b,b_out);

let mut buffer = String::new();

io::stdin().read_line(&mut buffer);


if buffer.trim() =="yes" {

balances.insert(token_a, users_bal_of_token_a-amout_of_a);

// if user have token_b => 
if balances.contains_key(&token_b){

    let prev_bal_of_b = balances.get(&token_b).unwrap();
    balances.insert(token_b, b_out+prev_bal_of_b);
    println!("swap successfull !!!");


}else {
    balances.insert(token_b, b_out);
    println!("swap successfull !!!");
}






}else {
println!("transaction calcelled");
}






}else{
println!("insufficient balance");
}








    
}
}






// wallet (address:String and balances:) 


// hashmap<Token, f64>  
// look up 

pub struct Wallet {

address:String,
pub balances:HashMap<Token,f64>,
}


impl Wallet {

// associted function
pub fn create_wallet(address:String, balances:HashMap<Token, f64>)-> Self {


Self { address, balances }


}

// method
pub fn check_my_balance(&self) {

// into_inter() -> ownership of vector 

println!("{:?}", self.balances); // hashmap-> 

}

}



// rahul'wallet -> usdt:1000 sol:40 , dot:9 Balances


// Balances (token: balance)


#[derive(Debug,Clone, Copy)]
pub struct Balances<T> {

pub token:T,
pub balance:f64

}

