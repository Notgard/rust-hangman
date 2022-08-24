# <span style="color:orange"> Rust Hangman </span> #

**Name:** Hangman  
**Version:** 0.2

This is a simple, terminal-based, hangman program made using the Rust programming language as a means to learn the basic functionalities of the language.   

I plan to latter fix up and improve performance of the code with further knowledge and experience I get from using Rust more.

## To play ##

First, be sure to clone the repository
```
git clone https://github.com/Notgard/rust-hangman.git
```

Afterwards, head into the cloned repository and run the package.
(Make sure to have rust and cargo installed)
```
cd rust-hangman
cargo run --package hangman --bin hagman
```
## Changelog ##

### 0.3 ###
TODO:
* Make file reading threaded
* Fix issues with player update
* Clean up code and lower memory usage
* Add some colors to the terminal

### 0.2 ###
* Ability to change manage players
* Added player scoreboard

### 0.1 ###
* Hangman game with manual, random and from word list input for the guessed word