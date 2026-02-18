use std::io;

fn main() {
    println!("X: ");

    let mut ans: String = String::new();

    io::stdin().read_line(&mut ans).expect("Failed to read line");

    println!("You entered: {}", ans.trim());

    let ha = 5;

    match ans.cmp(&ha.to_string()) {
        std::cmp::Ordering::Equal => println!("Equal"),
        std::cmp::Ordering::Greater => println!("Greater"),
        std::cmp::Ordering::Less => println!("Less"),
    }

}