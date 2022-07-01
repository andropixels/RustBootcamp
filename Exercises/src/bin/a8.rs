// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor



enum Flavor {

    Sweet,
    Spicy,
    Fruity,
    Salty

}

struct Drink {
    flavor:Flavor,
    fluid_ounce:f32
}

fn print_drink(drink:Drink) {

      match drink.flavor {
        Flavor::Fruity => println!("its fruity"),
        Flavor::Salty => println!("its salty"),
        Flavor::Spicy => println!("its spicy"),
        Flavor::Sweet => println!("its sweet"),
      }
      println!("{}", drink.fluid_ounce);

}



 





fn main() { 


let drink = Drink{
    flavor:Flavor::Sweet,
    fluid_ounce:3.2
};

print_drink(drink);


}


