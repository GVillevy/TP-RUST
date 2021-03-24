///////////////////////////////////////////////////////
//////           EXO SPAVANAC
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut mon_iterateur=input.split_whitespace();
    let heure_str = mon_iterateur.next().expect("Entier");
    let minutes_str = mon_iterateur.next().expect("Entier");

    let mut heure=heure_str.parse::<i32>().expect("Entier");
    let mut minutes=minutes_str.parse::<i32>().expect("Entier");

    if minutes<45 {
        let nouveau_minutes=45-minutes;
        if heure==0{
            heure=23;
        }else{
            heure=heure-1;
        }
        minutes = 60-nouveau_minutes;
    }else{
        minutes = minutes-45;
    }
    print!("{} {}",heure,minutes);

}
