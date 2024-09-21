use fast_typing::{Game, InputResult};
use std::io::{self, Write};

fn main() {
    let sample_text = String::from("Bonjour, monde ! Ceci est un jeu de dactylographie.");
    let mut game = Game::new(sample_text.clone());

    println!("Bienvenue dans le jeu de dactylographie !");
    println!("Tapez le texte suivant :");
    println!("------------------------\n{}\n------------------------", game.sample_text);
    println!("\nCommencez à taper :");

    let stdin = io::stdin();
    let mut input_buffer = String::new();

    while game.input_text.len() < game.sample_text.len() {
        // Lire l'entrée de l'utilisateur
        input_buffer.clear();
        stdin.read_line(&mut input_buffer).expect("Erreur de lecture de l'entrée");
        let input_line = input_buffer.trim_end();

        for input_char in input_line.chars() {
            let result = game.input(input_char);

            match result {
                InputResult::Success => {
                    // Afficher la progression
                    print!("\r{}", game.input_text);
                    io::stdout().flush().unwrap();
                }
                InputResult::Error => {
                    println!("\nErreur : Caractère incorrect '{}'. Attendu '{}'.",
                             input_char,
                             game.sample_text.chars().nth(game.input_text.len()).unwrap());
                }
                InputResult::Nothing => {
                    println!("\nPlus de saisie attendue.");
                    break;
                }
            }
        }
    }

    println!("\nFélicitations ! Vous avez terminé le jeu.");
}