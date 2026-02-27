use std::io::{self, Write};

fn is_prime(x : u32) -> bool {
    if x < 2 {
        return false
    }

    let mut i:u32 = 2;
    while i * i < x {
        if x % i == 0 {
            return false
        }

        i += 1;
    }

    true
}

fn get_number_ones(x : u32) -> u32 {
    let s = format!("{:b}", x);
    let mut count:u32 = 0;

    for c in s.chars() {
        if c == '1' {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut qty:u32 = 1;

    while qty > 0 {
        let mut input = String::new();

        print!("Enter a number: ");
        io::stdout().flush().unwrap(); // make sure prompt shows up

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let x: u32 = input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        qty =  get_number_ones(x);

        if qty < 2 {
            continue;
        }
        
        let s = format!("{:b}", x);

        if is_prime(qty) {
            println!("{} = {}: is prime", x, s);
        } else {
            println!("{} = {}: NOT is prime", x, s);
        }
    }

}
