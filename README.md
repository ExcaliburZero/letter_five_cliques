# Five five-letter words with twenty-five unique letters
A program that solves the problem that Matt Parker solved in his video "[Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)".

Based on [Benjamin Paassen's solution](https://gitlab.com/bpaassen/five_clique), but uses bit sets, memory re-use, and paralellization to speed up the runtime.

## Usage
```bash
# Get the dataset
wget https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt

# Run the program
cargo run --release -- words_alpha.txt
```