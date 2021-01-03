use bit_vec::BitVec;
use std::{io, process};
mod terminal;

pub struct Game {
    secret_word: String,
    guessed_letters: BitVec,
    incorrect_guesses: Vec<char>,
    pub output: String,
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
            incorrect_guesses: Vec::new(),
            output: String::with_capacity(GALLOWS_0.len()),
            guess: String::with_capacity(1),
            stdin,
            stdout,
        }
    }

    pub fn read_guess(&mut self) -> char {
        read("Guess", &mut self.guess, &self.stdin, &self.stdout);

        if self.guess == self.secret_word {
            self.guessed_letters.set_all();
            self.draw();
            self.output.push_str("Word was guessed!");
            quit(&self.output, 0);
        }

        let guessed_character = self.guess.chars().next().unwrap();

        self.guess.clear();

        let mut one_match = false;
        for (index, char) in self.secret_word.chars().enumerate() {
            if guessed_character == char {
                self.guessed_letters.set(index, true);
                one_match = true;
            }
        }

        if !one_match {
            for char in self.secret_word.chars() {
                if guessed_character != char && !self.incorrect_guesses.contains(&guessed_character)
                {
                    self.incorrect_guesses.push(guessed_character);
                    break;
                }
            }
        }

        guessed_character
    }

    pub fn draw(&mut self) {
        let mut hanged = false;
        let gallows = match self.incorrect_guesses.len() {
            0 => GALLOWS_0,
            1 => GALLOWS_1,
            2 => GALLOWS_2,
            3 => GALLOWS_3,
            4 => GALLOWS_4,
            5 => GALLOWS_5,
            _ => {
                hanged = true;
                GALLOWS_6
            }
        };

        self.output.push_str(gallows);

        let mut secret_word_characters = self.secret_word.chars();

        for guessed in &self.guessed_letters {
            let letter = secret_word_characters.next().unwrap();
            if guessed {
                self.output.push_str(terminal::UNDERLINE);
                self.output.push(letter);
                self.output.push_str(terminal::UNDERLINE_OFF);
                self.output.push(' ');
            } else {
                self.output.push_str("_ ");
            }
        }
        self.output.push('\n');

        for incorrect_guess in &self.incorrect_guesses {
            self.output.push_str(terminal::STRIKE);
            self.output.push(*incorrect_guess);
            self.output.push_str(terminal::STRIKE_OFF);
            self.output.push(' ');
        }
        self.output.push('\n');

        if hanged {
            self.output.push_str("Hangman was hanged!");
            quit(&self.output, 0);
        } else if self.guessed_letters.all() {
            self.output.push_str("Word was guessed!");
            quit(&self.output, 0);
        }

        println!("{}", self.output);
        self.output.clear()
    }
}

pub fn read_secret_word(stdin: &io::Stdin, stdout: &io::Stdout) -> (String, usize) {
    terminal::echo(stdin, false);

    let mut secret_word = String::with_capacity(1);

    let length = read("Secret word", &mut secret_word, &stdin, &stdout);

    terminal::echo(stdin, true);

    (secret_word, length)
}

fn read(
    thing: &str,
    /*into*/ buffer: &mut String,
    /*from*/ stdin: &io::Stdin,
    stdout: &io::Stdout,
) -> usize {
    let mut length;

    print!("{}: ", thing);
    terminal::flush(&stdout);
    loop {
        if stdin.read_line(buffer).is_ok() {
            *buffer = buffer.trim().to_string();
            length = buffer.chars().count();
            if length < 1 {
                print!(
                    "{} must be at least one character long!\n\
                     {}: ",
                    thing, thing
                );
                terminal::flush(&stdout);
                buffer.clear();
                continue;
            }
            break;
        }
    }

    length
}

fn quit(string: &str, code: i32) {
    println!("{message}", message = string);
    process::exit(code);
}

pub const GALLOWS_0: &str = "      ┌──────┐
      │╱     |
      │      |
      │
      │
      │
      │
_____╱│╲________
";
pub const GALLOWS_1: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │
      │
      │
_____╱│╲________
";
pub const GALLOWS_2: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /
      │
      │
_____╱│╲________
";
pub const GALLOWS_3: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|
      │
      │
_____╱│╲________
";
pub const GALLOWS_4: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │
      │
_____╱│╲________
";
pub const GALLOWS_5: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │     /
      │
_____╱│╲________
";
pub const GALLOWS_6: &str = "      ┌──────┐
      │╱     |
      │      |
      │      O
      │     /|\\
      │     / \\
      │
_____╱│╲________
";
