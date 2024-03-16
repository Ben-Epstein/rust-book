use rand::Rng;
use std::io;

fn more(a: i32) -> i32 {
    return a + rand::thread_rng().gen_range(1..=100);
}

fn fib(a: u32) -> u32 {
    if [0, 1].contains(&a) {
        return a;
    }
    return fib(a - 1) + fib(a - 2);
}

fn main() {
    println!("Give a number to get the fib sum:");
    let fib_val: u32 = {
        loop {
            let mut fib_val = String::new();
            io::stdin()
                .read_line(&mut fib_val)
                .expect("Failed to read line!");
            let fib_val: u32 = match fib_val.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{fib_val} not a number. Try again");
                    continue;
                }
            };
            break fib_val;
        }
    };
    let fib_sum = fib(fib_val);
    println!("Fib sum of {} = {}", fib_val, fib_sum);
    let fib_sum_more = more(fib_sum.try_into().unwrap());
    println!("more! {}", fib_sum_more)
}
