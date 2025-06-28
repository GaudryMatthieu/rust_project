
use std::io;

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

impl Livre {
    fn new(titre: &str, auteur: &str, annee: u32) -> Self {
        Livre {
            titre: titre.to_string(),
            auteur: auteur.to_string(),
            annee,
            disponible: true,
        }
    }
}

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();

    loop {
        println!("Bibliothèque");
        println!("1. Créer un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres dispo");
        println!("6. Quitter");

        let choix = lire_entree("Votre choix : ");

        match choix.trim() {
            "1" => ajouter_livre(&mut bibliotheque),
            "2" => emprunter_livre(&mut bibliotheque),
            "3" => retourner_livre(&mut bibliotheque),
            "4" => afficher_livres(&bibliotheque, false),
            "5" => afficher_livres(&bibliotheque, true),
            "6" => {
                println!("Ciao !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}

fn lire_entree(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    let _ = io::Write::flush(&mut io::stdout());
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn ajouter_livre(biblio: &mut Vec<Livre>) {
    let titre = lire_entree("Titre du livre : ");
    if biblio.iter().any(|l| l.titre == titre) {
        println!("Un livre avec ce titre existe déjà");
        return;
    }
    let auteur = lire_entree("Auteur : ");
    let annee_input = lire_entree("Année de publication : ");
    let annee: u32 = match annee_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Année invalide !!!!!!!");
            return;
        }
    };

    let livre = Livre::new(&titre, &auteur, annee);
    biblio.push(livre);
    println!("Livre ajouté !");
}

fn emprunter_livre(biblio: &mut Vec<Livre>) {
    let titre = lire_entree("Titre du livre à emprunter : ");
    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            if livre.disponible {
                livre.disponible = false;
                println!("Livre emprunté !");
            } else {
                println!("Le livre est déjà emprunté");
            }
            return;
        }
    }
    println!("Livre introuvable");
}

fn retourner_livre(biblio: &mut Vec<Livre>) {
    let titre = lire_entree("Titre du livre à retourner : ");
    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            if !livre.disponible {
                livre.disponible = true;
                println!("Livre retourné !");
            } else {
                println!("Le livre est déjà dispo");
            }
            return;
        }
    }
    println!("Livre introuvable");
}

fn afficher_livres(biblio: &Vec<Livre>, seulement_dispo: bool) {
    for livre in biblio.iter() {
        if !seulement_dispo || livre.disponible {
            println!(
                "titre: {} | auteur : {} | annee : {} | dispo : {}",
                livre.titre,
                livre.auteur,
                livre.annee,
                if livre.disponible { "oui" } else { "non" }
            );
        }
    }
}
