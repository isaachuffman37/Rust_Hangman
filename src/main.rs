use rand::prelude::*;
use std::process::Command;

fn main() {
    play_game();
}

fn get_guess() -> String {

    // Initialize variables
    let mut guess: String = String::new();
    let mut _response : String = String:: new();

    // Prompts user for the guess and puts response into 'guess' variable
    println!("What is your guess? ");
    std::io::stdin().read_line(& mut guess).unwrap();

    // Sheds off the newline character
    let _guess = guess.trim().to_string();

    // 'If' statement that handles if user input is more than one character
    if _guess.len() > 1 {

        // gets the first letter in the user input using a slice 
        let first_letter = &_guess[0..1];
        let _first_letter = first_letter.to_string();
        println!("Your guess contains more than 1 character, we will take the first letter of your guess.");

        // creates a loop until user responds with 'y' or 'n'
        while _response != "y" && _response != "n"{
            let mut _response = String::new();
            println!("Do you accept '{_first_letter}' as your answer? (y/n)");
            std::io::stdin().read_line(& mut _response).unwrap();
            let undercase_response = _response.trim().to_lowercase();

            // Prompts the user again and returns the new response
            if undercase_response == "n"{
                println!("No problem! Please enter your new guess.");
                println!("");
                return get_guess();

            // returns first letter if response is 'y'
            } else if undercase_response == "y"{
                return _first_letter;

            } else {
                println!("You must answer 'y' or 'n'");
                println!("")
            }

        }

    }
    return _guess;
}




fn is_done(stage:&usize, line:&Vec<char>)->bool{
    // returns false if stage is less than and is_not_guessed(line) returns true 
    if stage <&6 && is_not_guessed(line) == true{
        return false
    }
    // otherwise, return true
    return true
    
}


fn is_not_guessed(line:&Vec<char>)-> bool{
    // if the line contains any '_' characters, returns true
    for letter in line{
        if letter == &'_'{
            return true
        }
    }
    return false
}

fn display_wrong_guesses(_wrong_guesses: Vec<String>){
    // loops through each element in '_wrong_guesses' and prints them next to each other
    for guess in _wrong_guesses {
        print!("{guess}")
    }
}

fn clear(){
    // clears the console
    let _ = Command::new("cmd")
        .args(&["/C", "cls"])
        .status();
}


fn get_random_word(_word_arrray:[&str;5])-> &str {
    // gets a random number
    let rand_index = rand::thread_rng().gen_range(0.._word_arrray.len()+1);

    // gets the word at the random index and returns it
    let chosen_word: &str = _word_arrray[rand_index];
    return chosen_word;
}


fn display_stages(stage_number: usize){
    // ASCII representation of hangman from stages 0 to 6
    let stages = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n========", 
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n========", 
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n========", 
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n========", 
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n========", 
    ];

    // prints the stage at the index of stage_number
    println!("{}", stages[stage_number])
}


fn play_game(){
    // Presents user with instructions
    println!("Welcome to the Hangman Game! The program will generate");
    println!("a random word from a list, and you have to try to guess the letters.");
    println!("6 wrong guesses, and you are out!");
    println!("");

    // Initialize variables
    let mut wrong_guesses: Vec<String> = Vec::new();
    let mut line:Vec<char>= Vec::new();
    let mut stage: usize = 0;
    const GUESS_WORDS: [&str;5] = ["flying", "spiderman", "banana", "project", "animal"];
    let rand_word: &str = get_random_word(GUESS_WORDS);
    let mut guesses: Vec<String> = Vec::new();
    // Turns rand_word into a vec of chars to loop through
    let chars : Vec<char> = rand_word.chars().collect();

    // initializes the line under the ASCII hangman
    for _letter in rand_word.chars(){
        line.push('_');
    }

    // Keeps looping while game is not done
    while is_done(&stage, &line) == false{
        display_stages(stage);
        println!("");
        for letter in &line{
            print!("{letter}")
        } 
        println!("");
        print!("Wrong guesses: ");
        display_wrong_guesses(wrong_guesses.clone());
        println!("");
        println!("");

        // Gets guess from user and pushes it to guesses
        let guess: String = get_guess();
        guesses.push(guess.clone());

        for _guess in &guesses{

            // turns guess into a char
            let user_guess = _guess.chars().next().unwrap();
    
    
            if chars.contains(&user_guess){
    
                // loops a chars.len() number of times
                for i in 0..chars.len(){
            
                    // if char guess is the same as list of chars at i index, it will push guess to line at the same index
                        if chars[i] == user_guess{
                            line[i] = user_guess;
                        } 
                    
                    }
            } else if !chars.contains(&user_guess) && !wrong_guesses.contains(_guess) {
                stage += 1;
                wrong_guesses.push(_guess.clone());
            }
            clear();
        }




    }

    // prints success message
    if is_done(&stage, &line) && !is_not_guessed(&line){
        clear();
        display_stages(stage);
        println!("");
        println!("CONGRATULATIONS! You guessed the word! The word is: {}", &rand_word);
        println!("");

    // prints fail message 
    } else if is_done(&stage, &line) && is_not_guessed(&line){
        clear();
        display_stages(stage);
        println!("");
        println!("Awwww man. You did not guess the word. Your a hanged man!");
        println!("The word was: {}", &rand_word);
        println!("");

    }
}


