

fn main() {
    println!("*******************************************");
    println!("");
    // ****************************************************
    
    // guessing_game();

    // ****************************************************
    println!("");
    println!("*******************************************");
}

/* Guessing Game - Chapter 2
    fn guessing_game() {
        use std::io;
        use std::cmp::Ordering;
        use rand::Rng;
        println!("Guess the number!");
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100); // .. start inclusive, end exclusive | ..= start and end inclusive

        loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            }; // "Shadowing variable" Back assign an value to the same variable
            println!("Secret number: {}", secret_number);
            
            println!("Your guessed: {}", guess);
        
        
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }
    }
*/