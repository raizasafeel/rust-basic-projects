use rand::Rng;
use std::io;

fn main() {
    let words = ["cucumber", "aubergine", "coriander", "chives", "drumstick", 
    "capsicum", "zucchini", "okra", "kale", "asparagus", "leek", "brussels Sprouts"];

    let random_index = rand::thread_rng().gen_range(0..words.len());
    let random_word = &words[random_index];

    hang_man(random_word.to_string());

}

fn hang_man(word: String) {
    let mut mistake = 0;
    let mut current_word: Vec<char> = vec!['_'; word.len()];
    let hangman = ["", "\t 0\n\t\n\t", "\t 0\n\t |\n", "\t 0\n\t/|\n", "\t 0\n\t/|\\\n", "\t 0\n\t/|\\\n\t/", "\t 0\n\t/|\\\n\t/\\"];

    while loop {

    // TO DO: print 
        // println!("number of mistakes: {}", mistake);
        println!("hangman:\n{}", hangman[mistake]);
        println!("{:?}", current_word);

    // TO DO: take guess and turn guess into a char - check this
        println!("Enter an alphabet:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().chars().next().expect("Please enter a valid character");

    // check if guess is in word
        if word.contains(guess) {
            for (i, ch) in word.chars().enumerate() {
                if ch == guess {
                    current_word[i] = guess;
                }
            }
        }
        else {
            mistake = mistake + 1;
        }

        if mistake >= 6 {
            println!("hangman:\n{}", hangman[6]);
            println!("GAME OVER! YOU LOSE!");
            break;
        } 

        if mistake >= 6 || (!current_word.contains(&'_')){
            println!("You found the word!\n{:?}", current_word);
            break;
        } 
    
    }
        
}
