# bee
A simple [NYT Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) solver written in Rust.

![Bee Screenshot](https://github.com/Merkwurdichliebe/bee/blob/master/bee.jpg?raw=true)

## Usage

Enter a 7-letter sequence to display all possible words and pangrams.

## Finding maximum values

Enter 'maximum' to recursively find (rather slowly) the largest values for words, pangrams, score and ratio of pangrams to number of words.

There are 3,315,312,000 possible letter permutations.

## Required English dictionary

The executable expects to find `wordlist.txt` in the working directory, which can be downloaded from https://github.com/wordnik/wordlist. This is not the dictionary used by the NYT and will generate more valid words than the official solution.