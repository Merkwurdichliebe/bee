# bee
A simple [NYT Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) solver written in Rust.

![Bee Screenshot](https://github.com/Merkwurdichliebe/bee/blob/master/bee.jpg?raw=true)

## Usage

At the prompt, enter a 7-letter sequence (e.g. `aeinrst`) to display all possible words and pangrams, with the first character (`a` in this example) being the center letter of the puzzle.

## Finding maximum values

Instead of a 7-letter string, type `maximum` to find the letter combination with gives the maximum values for words, pangrams, score and ratio of pangrams to number of words.

The code uses a recursive function to check all 3,315,312,000 possible letter permutations. With the Wordnik dictionary, the process (single-threaded) takes approximately three hours to complete on a 4GHz i7 iMac.

## Required English dictionary

The executable expects to find `wordlist.txt` in the working directory, which can be downloaded from the [Wordnik Github repository](https://github.com/wordnik/wordlist). The words in the downloaded file should be stripped of their surrounding double-quotes (replaced with an empty string) before attempting to run the program.

Note: The Wordnik dictionary has many more words than the one used by the NYT Spelling Bee. It will therefore generate a greater number of valid results than the official puzzle solution.