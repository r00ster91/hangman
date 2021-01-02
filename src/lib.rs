use bit_vec::BitVec;
use std::{
    io::{self, Write},
    process,
};
mod terminal;

pub struct Game {
    secret_word: String,
    guessed_letters: BitVec,
    wrong_letters: Vec<char>,
    guess: String,
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Game {
    pub fn new(
        secret_word: String,
        secret_word_length: usize,
        stdin: io::Stdin,
        stdout: io::Stdout,
    ) -> Game {
        Game {
            secret_word,
            guessed_letters: BitVec::from_elem(secret_word_length, false),
            wrong_letters: Vec::with_capacity(1),

            guess: String::with_capacity(1),
            stdin,
            stdout,
        }
    }

    pub fn read_guess(&mut self) -> char {
        print!("Guess: ");
        self.stdout.flush();

        self.stdin.read_line(&mut self.guess);

        let guessed_character = self.guess.chars().next();

        self.guess.clear();

        let mut chars = &mut self.secret_word.chars();
        let mut correct = false;
        for (index, char) in chars.enumerate() {
            if guessed_character == Some(char) {
                self.guessed_letters.set(index, true);
                correct = true;
            }
        }

        if !correct {
            for char in self.secret_word.chars() {
                if guessed_character != Some(char)
                    && !self.wrong_letters.contains(&guessed_character.unwrap())
                {
                    self.wrong_letters.push(guessed_character.unwrap());
                    break;
                }
            }
        }

        guessed_character.unwrap()
    }

    pub fn draw(&self) {
        let mut game_over = false;
        let gallows = match self.wrong_letters.len() {
            0 => GALLOWS_0,
            1 => GALLOWS_1,
            2 => GALLOWS_2,
            3 => GALLOWS_3,
            4 => GALLOWS_4,
            5 => GALLOWS_5,
            _ => {
                game_over = true;
                GALLOWS_6
            }
        };

        println!();
        println!();
        println!("{}", gallows);

        let mut chars = self.secret_word.chars();

        for guessed in &self.guessed_letters {
            let letter = chars.next();
            if guessed {
                print!(
                    "{}{}{} ",
                    terminal::UNDERLINE,
                    letter.unwrap(),
                    terminal::UNDERLINE_OFF
                );
            } else {
                print!("_ ");
            }
        }
        println!();

        for wrong in &self.wrong_letters {
            print!("{}{}{} ", terminal::STRIKE, wrong, terminal::STRIKE_OFF);
        }
        println!();

        if game_over {
            println!("GAME OVER");
            process::exit(0);
        } else if self.guessed_letters.all() {
            println!("VICTORY");
            process::exit(0);
        }
    }
}

pub fn read_secret_word(stdin: &io::Stdin, mut stdout: &io::Stdout) -> (String, usize) {
    terminal::echo(stdin, false);

    let mut secret_word = String::with_capacity(1);
    let mut length;
    loop {
        print!("Secret word (echo off): ");
        stdout.flush();
        if stdin.read_line(&mut secret_word).is_ok() {
            secret_word = secret_word.trim().to_string();
            length = secret_word.chars().count();
            if length < 1 {
                println!("Secret word must be at least 1 character long!");
                secret_word.clear();
                continue;
            }
            break;
        }
    }

    terminal::echo(stdin, true);

    (secret_word, length)
}

pub const GALLOWS_0: &str = "
      ┌──────┐
      │╱     |
      │      |
      │
      │
      │
      │
_____╱│╲________
";
pub const GALLOWS_1: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │
      │
      │
_____╱│╲________
";
pub const GALLOWS_2: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /
      │
      │
_____╱│╲________
";
pub const GALLOWS_3: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|
      │
      │
_____╱│╲________
";
pub const GALLOWS_4: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │
      │
_____╱│╲________
";
pub const GALLOWS_5: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │     /
      │
_____╱│╲________
";
pub const GALLOWS_6: &str = "
      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │     / \\
      │
_____╱│╲________
";
