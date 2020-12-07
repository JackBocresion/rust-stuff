
pub fn vars1(){
let constant = 4;
//not mutable, with type infrence
const TYPEDEF:i32 = 5;
//consts shoul;d be all caps
let mut mutable = 6;
  println!("{:?}", (constant, TYPEDEF, mutable));
  //print a tuple in debug mode
}