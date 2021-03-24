///////////////////////////////////////////////////////
//////           EXO Detailed Differences
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut mon_iterateur=input.lines(); // lines = split('\n')
    
    let nombre_test=mon_iterateur.next().expect("Int");
    let nombre_test=nombre_test.parse::<i32>().expect("Entier");


    for _j in 0..nombre_test{

        let premiere_ligne=mon_iterateur.next().expect("String");
        let seconde_ligne=mon_iterateur.next().expect("String");

        let mut tableaupremiereligne = Vec::new();
        let mut tableausecondeligne = Vec::new();


        
        println!("{}",premiere_ligne);
        println!("{}",seconde_ligne);


        for letter in premiere_ligne.chars(){
            tableaupremiereligne.push(letter);
        }
        for letter in seconde_ligne.chars(){
            tableausecondeligne.push(letter);
        }


        for i in 0..tableaupremiereligne.len(){
            if tableaupremiereligne[i]!=tableausecondeligne[i]{
                print!("*");
            }else{
                print!(".");
            }
        }
        println!("");
    }
}