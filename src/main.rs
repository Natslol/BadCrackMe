use std::io;
use std::{thread, time::Duration};

fn main() {

    let x = ['h', 'i', 'd', 'e', 'a','k','i']; // C'est le mot de passe qui est fait de cette façon pour être difficile à deviner // c un array plus communement appelé tableau
    let finish = ["Correct password !", "Incorrect password !"]; // C'est les différentes fin possible

    let mut guessing = String::new(); // C'est pour définir que c'est une nouvelle string et de pouvoir l'écrire

    io::stdin().read_line(&mut guessing).expect("Can't read line"); // ça permet de de lire ce que j'ai écrit par exemple lire le "hideaki" que j'écrit dans la console

    let guessing: &str = guessing.trim(); // ça converti la String en &str

    let x: String = x.iter().collect(); // permet de convertir le tableau (qui est en char) en tableau String

    if !guessing.contains(&x[0..6]) { // ça la constance pour verifié le mdp par exemple le "!" que j'ai mis c pour dire que si "guessing" ne contient pas "&x[0..6]" (ce qui équivaut au tableau / "hideaki"  ) alors tu envoies ça
        println!("{}", finish[1]); // du coup ça envoie ça "finish[1]" signifie "Incorrect password"
        thread::sleep(Duration::from_millis(4000)); // temps d'attente avant la fermeture du programme
    } else { // else signe que si par exemple donc si alors guessing contient "hideaki" donc il envoie ça
        println!("{}", finish[0]);  // il va envoyé "Correct password !"
        thread::sleep(Duration::from_millis(4000)); // temps d'attente encore
    }
}