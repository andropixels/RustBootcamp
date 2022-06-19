fn main() {
    let mut count = 0_u32;

    println!("Lets count to infinitiy");

    loop {
        count = count + 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough"); 
            break;
        }
    }

    nested_loop();
    return_loop();
}

fn nested_loop() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        match act_up(counter) {
            Ok(r) => break r,
            Err(_) => continue
        }
    };

    assert_eq!(result, 5);
}

fn act_up(i: i32) -> Result<i32, i32> {
    match i {
        _ if i == 5 => Ok(i),
        _ => Err(i)
    }
}