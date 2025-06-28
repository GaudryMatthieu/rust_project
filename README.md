
# Rapport Technique du projet bibliothèque en Rust

---

## 🧱 Choix de Conception

### 1. Structure `Livre`

```rust
#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}
```

Utilisation d'une `struct` pour regrouper les informations d'un livre. Le champ `disponible` permet de suivre l'état du livre. (Imposer par le sujet)

---

### 2. Utilisation d’un `Vec<Livre>`

```rust
let mut bibliotheque: Vec<Livre> = Vec::new();
```

> Le sujet demandait explicitement un `Vec`, ce qui a été respecté.  
> Les opérations comme la recherche ou la modification passent par `iter_mut()` ou des boucles.

---

### 3. Boucle interactive avec `loop` et `match`

```rust
loop {
    println!("1. Créer un livre");
    // code
    match choix.trim() {
        "1" => ajouter_livre(&mut bibliotheque),
        "6" => break,
        _ => println!("Choix invalide"),
    }
}
```

Permet une interaction continue avec l'utilisateur jusqu'à ce qu'il quitte.

---

### 4. Validation des entrées

```rust
if biblio.iter().any(|l| l.titre == titre) {
    println!("Un livre avec ce titre existe déjà");
    return;
}
```

Empêche l’ajout de livres en double. Une vérification est aussi faite pour l’année.

---

## Fonctions principales

### Ajouter un livre

```rust
fn ajouter_livre(biblio: &mut Vec<Livre>) {
    let titre = lire_entree("Titre du livre : ");
    // encore du code
    biblio.push(Livre::new(&titre, &auteur, annee));
}
```

---

### Emprunter un livre

```rust
fn emprunter_livre(biblio: &mut Vec<Livre>) {
    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            // toujours plus de code pour modifier
        }
    }
}
```

---

### Retourner un livre

```rust
fn retourner_livre(biblio: &mut Vec<Livre>) {
    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            // again
        }
    }
}
```

---

### Afficher les livres

```rust
fn afficher_livres(biblio: &Vec<Livre>, seulement_dispo: bool) {
    for livre in biblio.iter() {
        if !seulement_dispo || livre.disponible {
            println!(...);
        }
    }
}

Merci pour la lecture et bonne correction :)
