//1
/* fn main() {
    println!("Hello World!");
} */

//2
/* compiler : rustc devine_mon_fichier.rs
   éxecuter : devine_mon_fichier */

// 3

/*  git remote add origin "lien vers depot git ou faire les commit"
    git init (le dossier entier a commit)
    git add . (tout add pour le commit)
    git commit -m "nom du commit"
    git push -u origin master (met a jour le github)
 */


//1

/* fn main(){
    println!("Devine mon nombre ! \n\nSaisissez votre proposition.");
} */

//2.ii
use std::io;  //import input

fn main(){
    println!("Devine mon nombre ! \n\nSaisissez votre proposition.");

    let mut input = String::new();

    io::stdin().read_line(&mut input);

    println!("Votre nombre est : {input}")
}

//2.iii
/* On fournit une variable prete à etre modifier (ici une reference mutable --> &mut) pour stocker la ligne lue*/

//2iv
 /* Result<usize> --> utilisé pour gerer les erreur --> moyen de retourner soit un succes (ici un usize) soit une erreur */
