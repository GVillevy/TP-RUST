///////////////////////////////////////////////////////
//////           EXO TIMELOOP
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn abracadabra(n:i32){
    for i in 1..=n{
        println!("{} Abracadabra",i);
    }

}

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let lines = input.lines();

    for line in lines{
        let n=line.parse::<i32>().expect("Entier");
        abracadabra(n);
    }
}