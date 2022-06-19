fn main() {
    // .. is end-exclusive
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    iter(&names);
    iter_mut(&mut names);
    into_iter(names); // consumes names    
}

fn iter(names: &Vec<&str>) {
    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn iter_mut(names: &mut Vec<&str>) {
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn into_iter(names: Vec<&str>) {
    for name in names.into_iter() { // starting from here name is moved
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}