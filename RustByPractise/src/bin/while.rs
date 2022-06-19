fn main() {
    fizz_buzz();
}


fn fizz_buzz() {
    let mut n = 0;

    while n < 20 {
        match n {
            _ if n % 15 == 0 => println!("fizzbuzz"),
            _ if n % 3 == 0 => println!("fizz"),
            _ if n % 5 == 0 => println!("buzz"),
            _ => println!("{}", n)
        }

        n += 1;
    }
}