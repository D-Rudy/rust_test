fn main() {
    
    //if
    let nombre = 6;
    
    //branches:
    // la condition doit retourner un booleen
    if nombre < 5 { 
        println!("Condition vérifiée");
    //Si pas de else le programme passe au lignes de code suivant le bloc if
    }else{    
        println!("Condition non vérifiée");
    }
    
    if nombre != 0 {
        println!("Le nombre a une valeur différente de zéro");
    }
    
    
    //Sort dès la première condition vérifiée
    //Il est plutot conseillé d'utiliser match
    if nombre % 4 == 0 {
        println!("Le nombre est divisible par quatre");
    }else if nombre % 3 == 0 {
        println!("Le nombre est divisible par trois");
    }else if nombre % 2 == 0 {
        println!("Le nombre est divisible par quatre");
    }else {
        println!("Le nombre n'est pas divisible par quatre, trois ou deux");
    }
    
    //If dans un let:
    //il faut que les types soit identiques
    let condition = true;
    let nombre = if condition { 5 } else { 6 };
    //let nombre = if condition { 5 } else { "six" }; ->Renvoi une erreur
    
    println!("La valeur du nombre est :{}", nombre);
    
    //Loop:
    let mut compteur = 0;
    
    //incrémente de 0 jusque 2
    'increment: loop {
        println!("Compteur = {}", compteur);
        let mut restant = 10;
        
        //décrémente de 10 à 9
        loop {
            println!("restant = {}", restant);
            if restant == 9 {
                break;// arrete la boucle sans étiquette
            }
            if compteur == 2 {
                break 'increment;//arrete la boucle avec l'étiquette 'increment
            }
            restant -= 1;
        }
        
        compteur += 1;
    }
    
    //retourner les valeur d'une boucle
    let mut compte = 0;
    
    let resultat = loop {
        compte += 1;
        
        if compte == 10 {
            break compte * 2;
        }
    };//Termine l'instrucion let ligne 66
    
    println!("le résultat est {}", resultat); 
    
    //boucle while:
    let mut chiffre = 3;
    
    //tant que la condition est vrai on execute le code
    //Sinon on quitte la boucle
    while chiffre != 0 {
        println!("{} !", chiffre);
        
        chiffre -= 1;
    }
    
    println!("DECOLLAGE!!!!!");
    
    //Boucler une collection avec for
    
    println!("Avec while:");
    
    //Avec while:
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;
    
    while indice < 5 {
        println!("La valeur est : {}", a[indice]);
        
        indice += 1;
    }
    
    println!("Avec for:");
    
    //Avec for: 
    for element in a {
        println!("La valeur est : {}", element);
    }
    
    for chiffre in (1..4).rev() {
        println!("{} !", chiffre);
    }
    println!("DECOLLAGE !!!!");
}
