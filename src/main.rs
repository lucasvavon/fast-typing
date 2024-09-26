use std::io;
use std::io::Write;
use std::io::{stdout};
use std::time::Instant;
use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand};
use crossterm::style::{PrintStyledContent, Stylize};
use rand::seq::SliceRandom;
use rand::thread_rng;
use fast_typing::{Game, InputResult};

fn main() -> io::Result<()> {
    let rand_text = get_rand_text();
    let mut game = Game::new(rand_text.clone());
    let mut start_time: Option<Instant> = None;

    // Enable raw mode to receive input without pressing Enter
    enable_raw_mode()?;

    println!("-------------------");
    println!("| FAST TYPING GAME |");
    println!("-------------------");
    println!("Type this series of words as quickly as possible : ");
    println!("{}", rand_text.clone());
    println!("-------------------");
    println!("Let's go : ");

    loop {
        if rand_text.len() == game.input_text.to_string().len() {
            break;
        }
        // Check if an event is available
        if event::poll(std::time::Duration::from_millis(100))? {
            // Read the next event
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        if start_time.is_none() {
                            start_time = Some(Instant::now());
                        }

                        match key_event.code {
                            KeyCode::Char(c) => {
                                let result = game.input(c);

                                match result {
                                    InputResult::Success => {
                                        stdout()
                                            .execute(PrintStyledContent(c.to_string().reset()))?;
                                    }
                                    InputResult::Error => {
                                        stdout()
                                            .execute(PrintStyledContent(c.to_string().red()))?;
                                    }
                                    InputResult::Jump => {
                                        let next_pos = game.jump_to_next_word();
                                        let dashes: String = std::iter::repeat("█").take(next_pos).collect();

                                        stdout()
                                            .execute(PrintStyledContent(dashes.red()))?;
                                    }
                                    InputResult::Nothing => {
                                        break;
                                    }
                                }
                                stdout().flush().unwrap();
                            }
                            KeyCode::Backspace => {
                                let result = game.input('\x08');
                                // Déplacer le curseur en arrière et effacer le caractère
                                stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                stdout().execute(PrintStyledContent(" ".to_string().reset()))?;
                                stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                stdout().flush().unwrap();

                                if let InputResult::Nothing = result {
                                    // Ne rien faire si aucun caractère à supprimer
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;

    // Calculer et afficher le temps écoulé
    if let Some(start) = start_time {
        let duration = start.elapsed();
        println!(
            "\nFélicitations ! Vous avez terminé le jeu en {:.2} secondes.",
            duration.as_secs_f64()
        );
    }
    Ok(())
}

fn get_rand_text() -> String {
    // A list of some English words
    let words = vec![
        "rust", "language", "code", "example", "random", "text", "generation", "function",
        "words", "crate", "simple", "program", "efficient", "system", "software"
    ];

    // Define how many words you want to generate
    let num_words = 10;

    // Create a random number generator
    let mut rng = thread_rng();

    // Select random words
    let random_text: Vec<&str> = words
        .choose_multiple(&mut rng, num_words)
        .cloned()
        .collect();

    // Join the random words into a sentence
    random_text.join(" ")
}
