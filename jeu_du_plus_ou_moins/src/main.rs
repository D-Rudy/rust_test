use rand::Rng; //Crate random
use std::cmp::Ordering;
use std::io; //Entrées/Sorties

fn main() {
    println!("Devines le nombre !");

    let nom_secret = rand::thread_rng().gen_range(1..101); //Génére un nombre aléatoire entre 1 et 100

    loop {
        println!("Entres un nombre : ");

        let mut supposition = String::new(); //La saisie est sous forme de chaine de caratères

        io::stdin() //Saisie utilisateur
            .read_line(&mut supposition) //référence
            .expect("Echec de la lecture de l'entrée utilisateur."); //Si erreur

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        println!("Ton nombre: {}", supposition);

        match supposition.cmp(&nom_secret) {
            Ordering::Less => println!("C'est plus"),
            Ordering::Greater => println!("C'est moins"),
            Ordering::Equal => {
                println!("Gagné !");
                break;
            }
        }
    }
}
