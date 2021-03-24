use std::io::{self,prelude::*};
///////////////////////////////////////////////////////
//////           EXO TP3
//////////////////////////////////////////////////////

fn is_odd(n:i32)-> bool {
    return n%2!=0
}

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    for line in lines{
        let n=line.parse::<i32>().expect("Entier");
        if is_odd(n){
            println!("{} is odd",n);
        }else{
            println!("{} is even",n);
        }
    }
}