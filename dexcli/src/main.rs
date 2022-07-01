use std::hash::Hash;
use std::io;

use std::collections::HashMap;
fn main() {
    
    println!("Please enter your address");

   let mut address = String::new(); //buffer -> address of user 
    io::stdin().read_line(&mut address);



    let tokenA = Balances { token:Token::SOL, balance:10000.0 };
    let tokenB = Balances { token:Token::DOT, balance:10000.0 };
   

    let mut  my_pool = Pool::create_pool(tokenA, tokenB);
     
    let mut balances_of_user = HashMap::new(); // wallet 

     balances_of_user .insert(Token::USDT,1000.0); 






// type -> value 
// type -> key -> hashable -> Hash & Eq
//implicitly hashmap    == ,type 

    // create a wallert for user 
      let mut users_wallet = Wallet::create_wallet(address,  balances_of_user);
    // wallet

       
      println!("your wallet created ");
      




      // swap 
      //uniswap 
      // pancakeswap 
      // binance 
      // sol => dot 
      // Liquidity pool == (TokenA TokenB)
      //SOL/USDT  => liquidity == (sol=10000, usdt=10000) balnces of tokens in the pool 
      //BTC/USDT => 
      //ETH/SOL => xETH => ySOL
      //X*Y=K  => 
      //creator of pool => 100ETH * 100SOL => 10000
      //K=10000
      //10ETH * YSOl = 10000
      //(100+10)ETH * (100-sol)=10000
      //110ETH*(100-sol) =10000
      //110-sol = 10,000/110
      //100-90.9  sol 
      // sol = 9.1


    // swap
    /// Liquidity POOL  (tokenA tokenB) => 
    /// tokenA=10000 tokenB=10000 
    /// x*y=k  => CREATION => K
    /// SOL/ETH =(100*100) => 10000
    /// (100+10)*(100-ETH) = 10000
    /// 110*100-ETH = 10000
    /// 100-ETH = 10000/110
    /// ETH =100-90.9
    /// ETH = 9.1
    /// slippage => 
    /// sol =>eth =>  
    /// 
    /// Constant product curve = x*y=k
    /// constant price
    /// offset 
    /// 
    
    






loop{
    println!("1-check your balance"); 
    println!("2- see the market"); 
    println!("3- buying token");
    println!("4- sell token");
    println!("5- swap  token");
    let mut  ch = String::new();
    io::stdin().read_line(&mut ch); 

    // 1 -> String -> int 
    // 1 -> i32 
    // trim -> white spaces -> &str 
   // unwrap -> Option or Result -> unwrap to get the value out of it 
    let choice:i32 = ch.trim().parse().unwrap();
      // matching on i32 => 
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

               

             // string = token_name :Token



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
     



    
      //  check_my_balance -> 







}  


mod MYMOD {



}

// listing tokens  -> 
#[derive(Debug,PartialEq,Hash,Eq,Clone, Copy)]
enum Token{

    SOL,
    DOT,
    ETH, 
    BTC, 
    USDT
}



// buy_token(token:Token, amount:f64, balances:Vec<Balances> )



impl Token {

    fn show_current_market() {
   // Self == TOken::
        println!("BTC: price:{}",Self::return_price(&Token::BTC) );
        println!("ETH: price:{}",Self::return_price(&Token::ETH) );
        println!("SOL: price:{}",Self::return_price(&Token::SOL) );
        println!("DOT: price:{}",Self::return_price(&Token::DOT) );
        println!("USDT: price:{}",Self::return_price(&Token::USDT) );
    }


    fn return_token(input:&str) -> Self {

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



    fn buy_token(self, amount:f64, balances:&mut  HashMap<Token,f64>) {

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

        fn sell_token(self,sell_token:Token,amount:f64,balances:&mut HashMap<Token,f64> ){

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
struct Pool {
    tokenA:Balances,
    tokenB:Balances
}

impl Pool {

    fn create_pool(tokenA:Balances,tokenB:Balances)-> Self{

        Self { tokenA, tokenB }

    }

    //direction 
    fn swap_a_to_b(&self,amout_of_a:f64,balances:&mut HashMap<Token,f64>) {

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

struct Wallet {

    address:String,
    balances:HashMap<Token,f64>,
}


impl Wallet {

// associted function
fn create_wallet(address:String, balances:HashMap<Token, f64>)-> Self {


    Self { address, balances }


}

// method
fn check_my_balance(&self) {
  
    // into_inter() -> ownership of vector 

    println!("{:?}", self.balances); // hashmap-> 

}

}



// rahul'wallet -> usdt:1000 sol:40 , dot:9 Balances


// Balances (token: balance)


#[derive(Debug,Clone, Copy)]
struct Balances {

    token:Token,
    balance:f64

}
