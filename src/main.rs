use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let (secret_word, secret_word_length) = hangman::read_secret_word(&stdin, &stdout);

    let mut game = hangman::Game::new(secret_word, secret_word_length, stdin, stdout);

    loop {
        game.draw();
        game.read_guess();
    }
}
