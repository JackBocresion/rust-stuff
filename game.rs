use std::cmp::Ordering;
use std::io;
//I assume cmp means compare?
use rand::Rng;

// entry point
pub fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("Guess the number");
    // macros
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        //const can be used for constants, but you cant use inferred types. 
        //2
        io::stdin()
            .read_line(&mut guess)
            //1
            .expect("Failed to read line");
        //shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number!");
                continue
            },
        };
        //match is basically a switch
        match guess.cmp(&secret_number) {
            // match is basically just a better C switch
            Ordering::Less => println!("Too little, try again"),
            Ordering::Greater => println!("Too high, try again"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
// rustfmt to format code!

/*
what did I learn?
- variables and mutability
- println! macro
- string concatination
- conventional declerations



*/
// 1
//the & in mut is a reference, which is a
//rust feature that makes it so memory safe.\

//2
//new string is convention rather than empty strings
/* like c and c++, string is of the standard lib. here we
are just defining lit so we dont need to case or shadow. */