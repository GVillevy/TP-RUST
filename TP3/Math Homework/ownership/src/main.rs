///////////////////////////////////////////////////////
//////           EXO MATH HOMEWORK mais il marche pas bien
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut mon_iterateur=input.split_whitespace();

    let premiere_animal = mon_iterateur.next().expect("Entier");
    let premiere_animal = premiere_animal.parse::<i32>().expect("Entier");

    let deuxieme_animal = mon_iterateur.next().expect("Entier");
    let deuxieme_animal = deuxieme_animal.parse::<i32>().expect("Entier");

    let troisieme_animal = mon_iterateur.next().expect("Entier");
    let troisieme_animal = troisieme_animal.parse::<i32>().expect("Entier");

    let objectif_patte = mon_iterateur.next().expect("Entier");
    let objectif_patte = objectif_patte.parse::<i32>().expect("Entier");

    let mut reponse : bool = false;

    if 0<premiere_animal && premiere_animal<=100 && 0<deuxieme_animal && deuxieme_animal<=100 && 0<troisieme_animal && troisieme_animal<=100 && 0<=objectif_patte && objectif_patte<=250{

        for _i in 0..100{
            for _j in 0..100{
                for _k in 0..100{
                    if _i*premiere_animal + _j*deuxieme_animal+_k*troisieme_animal == objectif_patte{
                    println!("{} {} {} ",_i,_j,_k);
                    reponse = true;
                    }
                }
            }
        }
    }

    if reponse == false {
        println!("impossible");
    }
}