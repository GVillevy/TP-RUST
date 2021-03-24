use std::io::{self,prelude::*};
///////////////////////////////////////////////////////
//////           EXO TP4
//////////////////////////////////////////////////////
fn main(){
    //PREMIERE METHODE

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let my_string = input;
    let mut my_vector = Vec::new();
    for letter in my_string.chars(){
        my_vector.push(letter);
    }
    my_vector.reverse();
    for element in my_vector{
        print!("{}",element);
    }
    println!("");



    //DEUXIEME METHODE

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let my_string = input;

    let mut vector = my_string.chars().collect::<Vec<char>>();

    vector.reverse();

    let monstring = vector.iter().collect::<String>();

    println!("{}",monstring);


}