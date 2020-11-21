use std::io;

fn main() {
    let x = 5;
    println!("Hello, world!");
    for i in 0..5 {
        println!("{}", i);
    }

    loop {
      let mut guess = String::new();
      io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
      println!("Here's your guess - {}", guess);
    }

    println!("XX");
}
