// Topic: Getting Used to with struct
//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle 
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape 
 7-Create a function which takes Shape and prints the dimension of all shapes 
*/

struct Shape {
    square: Square,
    rectangle: Rectangle,
    }
    

#[derive(Debug)]
    struct Square {
    side: i32,
    }


    impl Square {
        fn new_square(side:i32) -> Self {
            Self { side }
        }

        fn print_square(&self) {
            println!("{}",self.side);
        }
        
    }
    
#[derive(Debug)]
    struct Rectangle {
    length: i32,
    width: i32,
    }
    

    impl Rectangle {

        fn new_rect(length:i32, width:i32 ) -> Self { // assocaited function

            return Self { length, width } ; // Self -> type (Rectangle)
         
         }

         fn print_rect(&self) {  // &self -> instance of type // method


            println!("{} {} ", self.length,self.width);
         }
    }


    






impl Shape {
  
    fn new_shape(square:Square,rectangle:Rectangle) -> Shape {

        Shape { square:square, rectangle:rectangle }
    }
         
    

   fn print_shape(&self) {
        println!("{:?} {:?}" , self.rectangle,self.square);
    }
}




fn print_shape(shape:Shape) {

    println!("{:?} {:?} ", shape.rectangle,shape.square);

}




fn main() {
    
  let new_rect = Rectangle::new_rect(4,6, );
  new_rect.print_rect();
  let square_instace =Square::new_square(8);
  square_instace.print_square();
  let shape = Shape::new_shape(square_instace, new_rect);
  shape.print_shape();

} 







