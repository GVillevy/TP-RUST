///////////////////////////////////////////////////////
//////           EXO VOLIM
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut mon_iterateur = input.lines();
    let num_joueur = mon_iterateur.next().expect("Entier");
    let mut num_joueur=num_joueur.parse::<i32>().expect("Entier");


    let nombre_question = mon_iterateur.next().expect("Entier");
    let nombre_question=nombre_question.parse::<i32>().expect("Entier");

    let mut temps_total : i32=0;


    for _i in 0..nombre_question{

        let current_question = mon_iterateur.next().expect("String");

        let mut mon_deuxieme_iterateur=current_question.split_whitespace();

        let temps = mon_deuxieme_iterateur.next().expect("Entier");
        let temps = temps.parse::<i32>().expect("Entier");
        let reponse = mon_deuxieme_iterateur.next().expect("String");
        let mut autre_iterateur = reponse.chars();
        let vrairep=autre_iterateur.next().expect("String");

        temps_total=temps_total+temps;


        if temps_total<=210{
        if vrairep=='t' || vrairep=='T'{
            num_joueur=num_joueur+1;
            if num_joueur==9{
                num_joueur=1;
            }
        }
    }
    }
    println!("{}",num_joueur);
}