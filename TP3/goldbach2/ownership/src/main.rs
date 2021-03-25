use std::io::{self,prelude::*};

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let _nombre=input.parse::<i32>().expect("Entier");
    let mut val=1;
    let mut compteur=0;

    for _i in 0..100{
        while val <=100{
            if _i%val==0{
                compteur+=1;
            }
            val+=1;
        }
        if compteur==2{
            println!("{}",_i);
        }
    }


}