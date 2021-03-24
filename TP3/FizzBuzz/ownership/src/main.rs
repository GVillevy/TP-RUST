///////////////////////////////////////////////////////
//////           EXO FIZZBUZZ
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn fizzbuzz(x:i32,y:i32,n:i32){
    for i in 1..n+1{
        if i%x==0 {
            print!("Fizz");
        }
        if i%y==0 {
            print!("Buzz");
        }
        if i%y!=0 && i%x!=0 {
            print!("{}",i);
        }
        println!("");
    }
}

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut my_iterator = input.split_whitespace();

    let x = my_iterator.next().expect("Entier");
    let x=x.parse::<i32>().expect("Entier");

    let y = my_iterator.next().expect("Entier");
    let y=y.parse::<i32>().expect("Entier");
    
    let n = my_iterator.next().expect("Entier");
    let n=n.parse::<i32>().expect("Entier");

    fizzbuzz(x,y,n);

}