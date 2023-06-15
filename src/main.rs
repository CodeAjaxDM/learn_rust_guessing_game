use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess teh number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop
    {
        println!("please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("you guess: {guess}");

        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("You Win!!"); break;},
            Ordering::Greater => println!("Too Big"),
        }
    }
}
