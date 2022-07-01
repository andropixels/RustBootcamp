// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print







fn main() {


    let which_color = Color::which_color("red");


    let red = Color::Red; 
    red.print_color();
    //    Color::print_color(&which_color);
    which_color.print_color(); // method





}
 


enum Color {

    Red,
    Violet,
    Pink,
    Black
}

impl Color {

    fn which_color(input:&str) -> Self {  // associated functiom
        match input{
              "red"=> Color::Red,
              "pink"=> Color::Pink,
              "black" => Color::Black,
              "violet"=> Color::Violet,
              _ =>  Color::Black
        }
    }

    fn print_color(&self) { // method
        match self {
            Color::Black => println!("black"),
            Color::Pink => println!("pink"),
            Color::Red => println!("red"),
            Color::Violet => println!("violet"),
        }
    }
    

}





