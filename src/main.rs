use std::{fs, io};
use std::io::{Write};
use std::io::{stdout};
use std::time::Instant;
use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand};
use crossterm::style::{PrintStyledContent, Stylize};
use rand::seq::SliceRandom;
use rand::thread_rng;
use console::{pad_str, Alignment, Term};
use terminal_size::{Width};
use fast_typing::{Game, InputResult};

fn main() -> io::Result<()> {
    let rand_text = get_rand_text_from_json("en");
    let mut game = Game::new(rand_text.clone());
    let mut start_time: Option<Instant> = None;

    // Enable raw mode to receive input without pressing Enter
    enable_raw_mode()?;

    let term = Term::stdout();
    let (width, height) = Term::size(&term);
    let width_usize: usize = width.into();
    let title = pad_str("FAST TYPING GAME", width_usize, Alignment::Center, None);

    println!("{}\n", title);
    println!();
    println!("Type this series of words as quickly as possible:");
    println!("{}\n", rand_text.clone());
    println!("-------------------\n");
    println!("Let's go : \n");

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
                                let jump_size = game.back_to_previous_position();
                                if jump_size > 1 {
                                    game.input('\x08');
                                    stdout().execute(crossterm::cursor::MoveLeft(jump_size as u16))?;
                                    stdout().execute(PrintStyledContent(" ".repeat(jump_size).reset()))?;
                                    stdout().execute(crossterm::cursor::MoveLeft(jump_size as u16))?;
                                } else {
                                    game.input('\x08');
                                    stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                    stdout().execute(PrintStyledContent(" ".to_string().reset()))?;
                                    stdout().execute(crossterm::cursor::MoveLeft(1))?;
                                }

                                stdout().flush()?;
                            }
                            KeyCode::Esc => {
                                println!("\nGAME OVER\n");
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    if let Some(start) = start_time {
        let duration = start.elapsed();
        println!(
            "\nCongratulations! You've completed the game in {:.2} seconds.",
            duration.as_secs_f64()
        );
        println!(
            "| Nb errors : {} |",
            game.count_errors()
        );
    }
    Ok(())
}

fn get_rand_text_from_json(lang: &str) -> String {
    let file = [lang, "json"].join(".");
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