# Five five-letter words with twenty-five unique letters
A program that solves the problem that Matt Parker solved in his video "[Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)".

Based on [Benjamin Paassen's solution](https://gitlab.com/bpaassen/five_clique), but uses bit sets, memory re-use, and paralellization to speed up the runtime. Additionally my program is written in Rust.

## Comparison
I got the following results by running the scripts on my PC (12 CPU cores, AMD Ryzen 5 3600). The time values were from running the script via the `time` command and grabbing the value labeled "total". Runtimes were rounded up to the nearest second.

The runtimes for Benjamin Paassen's script include both the time to generate the graph and the time to find the cliques (`generate_graph.py` and `five_clique.py`).

| Dataset | My script | Benjamin Paassen's script | Speedup |
|---------|-----------|---------------------------|---------|
| words_alpha.txt | 38s | 19m 6s | ~30x |
| Wordle Answers + Guesses | 18s | 8m 26s | ~28x |

## Usage
```bash
# Get the dataset
wget https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt

# Run the program
cargo run --release -- words_alpha.txt
```