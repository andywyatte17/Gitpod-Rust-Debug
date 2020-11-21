use std::io;
use std::io::BufRead;

fn main() {
    let cursor = io::Cursor::new(b"1\n2\n123\n");
    let mut line_reader = io::BufReader::new(cursor);

    println!("YY");

    loop {
      let mut guess = String::new();
      print!("Please type in your guess: ");
      //io::stdin().read_line(&mut guess)
      //  .expect("Failed to read line");
      line_reader.read_line(&mut guess)
        .expect("Failed to read line");
      println!("Here's your guess - {}", guess);
      if guess=="123" || guess=="123\n" {
          break;
      }
    }

    println!("XX");
}
