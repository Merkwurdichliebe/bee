# bee
A simple [NYT Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) solver written in Rust.

![Bee Screenshot](https://github.com/Merkwurdichliebe/bee/blob/master/bee.jpg?raw=true)

## Usage

Enter a 7-letter sequence (e.g. `aeinrst`) to display all possible words and pangrams, with the first character (`a` in this example) being the center letter of the puzzle.

## Finding maximum values

Enter 'maximum' to recursively find the largest values for words, pangrams, score and ratio of pangrams to number of words.

There are 3,315,312,000 possible letter permutations. With the Wordnik dictionary, the process (single-threaded) runs for approximately three hours on a 4GHz i7 iMac.

## Required English dictionary

The executable expects to find `wordlist.txt` in the working directory, which can be downloaded from https://github.com/wordnik/wordlist. This is not the dictionary used by the NYT and will generate more valid words than the official solution.