fn main() {

//Variables préfixées par underscore pour éviter les warning

    //Entier non signé sur 32bits (scalaire):
    let _a: u32 = 42;

    //Entier signé sur 64bits (scalaire):
    let _b: i64 = -23658457;

    //Flottant (scalaire):
    let _c = 2.0; //f64
    let _d: f32 = 3.0;

    //Booléen (scalaire):
    let _e = true;
    let _f: bool = false; //annotation explicite

    //Caractère (scalaire):
    let _g = 'z';
    let _h = 'ℤ';
    let _chat_aux_yeux_de_coeur = '😻';

    //Addition:
    let _somme = 5 + 10;

    //Soustraction:
    let _difference = 95.5 - 69.2;

    //Multiplication:
    let _produit = 4 * 8;

    //Division:
    let _quotient = 56.7 / 32.1;
    let _arrondi = 2 / 3; //retourne 0;

    //Modulo:
    let _reste = 43 % 5;

    //Tuple (composé):
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Destructurer un tuple:
    let (x, y, z) = tup;
    println!("la valeur de x est : {}", x);
    println!("la valeur de y est : {}", y);
    println!("la valeur de z est : {}", z);

    //Accès par point(.):
    let _i: (i32, f64, u8) = (500, 6.4, 1);
    let _cinq_cent = _i.0;
    let _six_virgule_quatre = _i.1;
    let _un = _i.2;

    //Tableau (composé):
    let _j = [1,2,3,4,5];
    let _mois = ["janvier", "février", "mars", "avril", 
                 "mai", "juin", "juillet", "aout",
                 "septembre", "octobre", "novembre", 
                 "décembre"];

    //Tableau d'entier de 5 indices:
    let _k: [i32; 5] = [1,2,3,4,5];

    //Créer un tableau de 5 indices contenant tous le chiffre 3:
    let _l = [3; 5];

    //Accès aux élément d'un tableau:
    let _premier = _l[0];
    let _dernier = _l[4];

    let _out_of_bounds = _l[10]; //Panic
}
