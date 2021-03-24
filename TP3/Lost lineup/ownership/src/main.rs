///////////////////////////////////////////////////////
//////           EXO LOST LINEUP
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut mon_iterateur = input.lines();
    let nombre_personne = mon_iterateur.next().expect("Entier");
    let nombre_personne=nombre_personne.parse::<i32>().expect("Entier");
    let place = mon_iterateur.next().expect("String");

    let mut mon_vector=place.chars().collect::<Vec<char>>();
    mon_vector.reverse();

    let lesplaces=mon_vector.iter().collect::<String>();
    let mut mon_deuxieme_iterateur = lesplaces.split_whitespace();
    print!("1 ");
    for _i in 0..nombre_personne-1{
        let a = mon_deuxieme_iterateur.next().expect("Entier");
        let a = a.parse::<i32>().expect("Entier");
        print!("{} ", nombre_personne-a);
    }
}