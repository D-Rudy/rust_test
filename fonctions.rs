fn main() {
    println!("Hello, world!");

    let x = 5;

    let y = {
        let x = 3; // masque sur x, la valeurs n'est valable que dans ce scope
        x + 1 //Les expressions n'ont pas de point virgule
    };

    une_autre_fonction(x, y);

    let cinq = plus_cinq(5);

    println!("la fonction plus_cinq retourne : {}", cinq);
}

fn une_autre_fonction(x: i32, y: i32) {
    println!("la valeur de x est : {}", x); //x vaut 5
    println!("la valeur de y est : {}", y); // y vaut (x = 3 + 1)
}

//Fonction retournant un integer
fn plus_cinq(x: i32) -> i32 {
    x + 5
}
