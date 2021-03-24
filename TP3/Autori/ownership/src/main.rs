///////////////////////////////////////////////////////
//////           EXO AUTORI mais je l'ai pas fini j'ai la flemme
//////////////////////////////////////////////////////
use std::io::{self,prelude::*};

fn main(){
    /*
    let mut input = String::new(); // on crée une variable input de type string
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin"); // on associe le continue en parametre à notre variable input

    let my_string = input; // on donne le résultat de input à une nouvelle variable
    let mut my_iterator = my_string.split('-'); // on split notre string par des -

    let premierprenom = my_iterator.next().expect("String"); // on sélectionne notre premiere partie avec le premier - et on l'ajoute à une var
    let secondprenom = my_iterator.next().expect("String"); // on sélectionne notre seconde partie avec le premier - et on l'ajoute à une var
    println!("{}",premierprenom);
    println!("{}",secondprenom);


    let mut my_second_iterator = premierprenom.chars();
    let premierelettre = my_second_iterator.next().expect("char");
    println!("{}",premierelettre);
    let mut my_second_iterator = secondprenom.chars();
    let premierelettre = my_second_iterator.next().expect("char");
    println!("{}",premierelettre);
    */

    let mut input = String::new(); // on crée une variable de type String qui s'apelle input
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin"); // input prend la valeur du input

    let mut un_iterator = input.split('-'); // on split avec des -
    let premiere_partie = un_iterator.next().expect("String"); // on prend la première partie de l'itérateur
    let seconde_partie = un_iterator.next().expect("String"); // on prend la seconde partie de l'itérateur 
    println!("{}",premiere_partie);
    println!("{}",seconde_partie);



}
