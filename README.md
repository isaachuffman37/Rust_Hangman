# Overview

This Rust program lets the user play the popular game "Hangman" in the terminal. It displays a message to the user describing the game's rules and chooses a random word from a list that the player must guess. If the user enters a guess that is more than one character, it doesn't just prompt the user again, it slices the guess to get the first letter of that guess and then asks if the user wants that letter to be their guess. If not, the program prompts the user for another guess. If the user's guess is in the random word, it displays the guess in place of the dash on the dashed line. If it is not in the random word, the program displays the next stage of the ASCII representation of hangman and displays the wrong guess to the user. Once the player guesses all of the letters in the word, it displays the word and a congratulatory message. If they guess incorrectly six times, the program will display all of the parts of the "hanged man" and displays a discouraging message to the user. 


This personal project helped stretch my ability to learn new and difficult programming languages. I wanted to test my ability to problem-solve and implement my knowlege of programming in a language that I have never seen or used. I also wanted to challenge myself by choosing a language that uses concepts that are unique to other languages (in this case, using the concept of ownership to help prevent memmory errors).


[LINK TO DEMO VIDEO](https://youtu.be/LHUgrTcDd8o)

# Development Environment

I used VSCode and a few libraries from the language. I used the "rand" library so that I could generate a random index and the "Command" library so that I could clear the console.  


# Useful Websites

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Tutorials Point](https://www.tutorialspoint.com/rust/index.htm)

# Future Work

- Strengthen the cohesion of my functions ( especially get_guess()) so they only do one thing.
- Increase the level of abstraction by taking some of the logic of "Hangman" in the function "play_game()" and making them into separate funtions that "play_game()" will call. This will strengthen cohesion as well. 
- Adjust my program layout and syntax to what is typical for Rust
- Allow the user to save the progress of the game by writing the list of guesses to a file and reading it again into the program. 

