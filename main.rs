mod prints;
mod vars;
mod game;
use std::io;
//mod stands for module
fn main() {
  loop{
  let mut input = String::new();
  println!("pls gimme an integer to make binaree");
  io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

   match input.trim().parse() {
            Ok(num) => {prints::print_binary_num(num);
            break;
            },
              Err(f) => {
                println!("please enter a valid number! {}",f);
                continue;
                }
            }
  }
  //vars::vars1();
  //game::main()
  //rust compiler likes to bitch about conventions :lol:
}