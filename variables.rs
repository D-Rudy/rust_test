fn main() {
    //Déclaration de constante:
    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;

    //pour réassigner une variable,
    //on utilise le mot-clé mut
    //let mut x = 5;
    let x = 5;
    println!("Valeur de x : {}", x);

    //sinon erreur de compilation:
    //x = 6;

    //fonctionne en créant un masque:
    let x = x + 1;

    println!("Valeur de x : {}", x);

    {
        let x = x * 2;
        println!("Valeur de x dans la portée interne : {}", x);
    }

    // x n'a été modifié que dans la portée interne:
    println!("Valeur de x : {}", x);

    println!("valeur de 3h en seconde : {}", TROIS_HEURES_EN_SECONDES);

    let lettres = "12345";
    println!("valeur de lettres en chaine de caratères : {}", lettres);

    let lettres = lettres.len();
    println!("valeur de lettres après réassignement : {}", lettres);

    //On ne peut pas muter le type d'une variable:
    /*
    let mut lettre = "12345";
    println!("valeur de lettre (mut) en chaine de caratères : {}",lettre);

    lettre = lettre.len();
    println!("valeur de lettre (mut) après réassignement : {}",lettre);
    */
}
