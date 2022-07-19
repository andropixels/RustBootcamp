use std::io;



mod mychain;

use mychain::Chain; 

fn main() {
 
    println!("enter the miner address")   ;

    let mut miner_addr = String::new();

    io::stdin().read_line(&mut miner_addr);


    println!("enter the difficulty")   ;
    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty);
    
    let parsed_difficulty:u32 = difficulty.trim().parse().unwrap();

   println!("generating genesis block! 
   ");

   // instance of chain 
   // genesis block 
   // miner addd 
   // diffi
   // Vec transactions 
   // merkle 
   // time_stamp
   // nonce
   // 

   let chain = Chain::new(miner_addr.as_str(),parsed_difficulty,100.0);

 

    



}



