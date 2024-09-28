use std::{fs, io};
use std::io::{Write};
use std::io::{stdout};
use std::path::PathBuf;
use std::time::Instant;
use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand};
use crossterm::style::{PrintStyledContent, Stylize};
use rand::seq::SliceRandom;
use rand::thread_rng;
use fast_typing::{Game, InputResult};

fn main() -> io::Result<()> {
    let lang = std::string::String::from("fr");
    let rand_text = get_rand_text_from_json(lang);
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
                                game.input('\x08');
                                stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                stdout().execute(PrintStyledContent(" ".to_string().reset()))?;
                                stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                stdout().flush().unwrap();
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

fn get_rand_text_from_json(lang: String) -> String {
    let file = [lang, String::from("json")].join(".");
    let data = fs::read_to_string(file).expect("REASON");
    let mut words: Vec<String> = serde_json::from_str(&data).expect("REASON");

    if words.len() < 20 {
        panic!("Not enough words in the list.");
    }

    // Shuffle the words
    let mut rng = thread_rng();
    words.shuffle(&mut rng);

    // Get a slice of the first 20 words
    let sample = &words[..20];

    sample.join(" ")
}