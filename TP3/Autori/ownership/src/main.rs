///////////////////////////////////////////////////////
//////           EXO AUTORI
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mon_iterateur=input.split('-');

    for mots in mon_iterateur{
        let mut mon_second_iterateur=mots.chars();
        let initiale=mon_second_iterateur.next().expect("String");
        print!("{}",initiale);
    }
}
