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
    for i in 2..100 {
        let qty:u32 =  get_number_ones(i);
        let s = format!("{:b}", i);

        if is_prime(qty) {
            println!("{} = {}: is prime", i, s);
        } else {
            println!("{} = {}: NOT is prime", i, s);
        }
    }
}
