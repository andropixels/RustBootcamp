use std::io;



mod mychain;

use mychain::Chain; 
// adding 
// pow 
// 

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

   let mut  chain = Chain::new(miner_addr.as_str(),parsed_difficulty,100.0);

  loop{
    println!("Menu");
    println!("1-New Transaction");
    println!("2-Mine Block");
    println!("3-Change difficulty");
    println!("4-Change Reward");
    println!("0-Exit");
 
    let mut choice = String::new();
    io::stdin().read_line(&mut choice); 
 
    // 
    let parsed_choice:u32 = choice.trim().parse().unwrap() ;
    match parsed_choice {
 
 
               1 => {


    let mut sender  = String::new() ;
    let mut  rec =  String::new() ;
    let mut amount = String::new();
 
                 // add a transaction 
               
                 io::stdin().read_line(&mut sender);
 
       
                 io::stdin().read_line(&mut rec);
 
 
                // let a  = convert_to_ref(sender);
                 io::stdin().read_line(&mut amount);
                   // dangle 
                 let parse_amount:f32 = amount.trim().parse().unwrap();
   
                 let res = chain.add_transaction(sender, rec, parse_amount);
 
                 match res {
 
                     Ok( _) =>{
                         println!("Transaction is added");
                     },
                     Err(_) =>{
                         println!("Transaction is reverted");
                     }
                 }
 
               },
 
               2=>{
 
                 // mine a block => generating block
                 println!("generating a block"); 
                 let res = chain.generate_new_block();
 
 
              match res {
 
                     Ok( _) =>{
                         println!("block is added");
                     },
                     Err(_) =>{
                         println!("block is reverted");
                     }
                 }
 
               },
               3=>{
                 // change difficulty 
                 let mut new_difficulty = String::new() ;
                 // method 
 
                 io::stdin().read_line(&mut new_difficulty );
                    let parsed_new_difficulty:u32 = new_difficulty.trim().parse().unwrap();
 
                     let res = chain.change_diff(parsed_new_difficulty);
 
                     match res {
 
                         Ok( _) =>{
                             println!("difficulty changed");
                         },
                         Err(_) =>{
                             println!(" there is a issue changing difficulty");
                         }
                     }
 
               },
               4=> {
                 let mut new_reward = String::new() ;
                 io::stdin().read_line(&mut new_reward );
                 let parsed_new_reward:f32 = new_reward.trim().parse().unwrap();
 
                  let res = chain.change_reward(parsed_new_reward);
 
                  match res {
 
                      Ok( _) =>{
                          println!("Reward changed");
                      },
                      Err(_) =>{
                          println!(" there is a issue changing Reward");
                      }
                  }
                 // change reward
               }
               _=> {
                 // exit 
 
                 println!("exiting ..... ")
               }
 
    }
 

  }
  


    



}


// fn add_trans(c:&mut Chain) {

//     let mut sender  = String::new() ;
//     let mut  rec =  String::new() ;
//     let mut amount = String::new();

//     io::stdin().read_line(&mut sender);
 
       
//     io::stdin().read_line(&mut rec);


//    // let a  = convert_to_ref(sender);
//     io::stdin().read_line(&mut amount);
//       // dangle 
//     let parse_amount:f32 = amount.trim().parse().unwrap();

//     let res = c.add_transaction("a", rec.as_str(), parse_amount);

//     match res {

//         Ok( _) =>{
//             println!("Transaction is added");
//         },
//         Err(_) =>{
//             println!("Transaction is reverted");
//         }
//     }
// }


// fn convert_to_ref<'a>(s:String) -> &'a str {
// // s  
//     s.as_str() // dangle 
// }// 

// fn get_sender() -> &str {

// }


