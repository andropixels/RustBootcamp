// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

use std::fmt::Debug;


enum Z{
    Data(Ordinates)
}


enum Ordinates{
    Data(i32,i32)
 }

// Fill in the blank
fn add<A:PartialEq>(a:A,b:A)-> bool  {

    if b!=a {
        return false;
    }else {
        return true ; 
    }



}


fn some<A:Debug+Copy,B:Debug+Copy>(mut a:A,b:B) {
    let c:B ; 
    let d:A;

   a = d; 
   c=b; 
   
    
    println!("{:?} {:?}", a,b);
    println!("{:?} {:?}", c,d);

}


enum Message {
    ChangeColor(i32,i32,i32),
    Echo(String),
    Move(Point),
    Quit
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
    }
}


fn main() {
  // call the process function 
  some::<i32,f32>(3,4);

} 
