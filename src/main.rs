use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    io::{BufRead, Write},
};

use bit_set::BitSet;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    println!("Reading in words file...");
    let words = read_words_file("data/wordle-all.txt").unwrap();
    println!("words.len() = {}", words.len());

    let letter_frequencies = calculate_letter_frequencies(&words);
    for (letter, count) in letter_frequencies.iter() {
        println!("{} = {}", letter, count)
    }

    println!("Building up the graph...");
    let graph = Graph::build_from_words(&words);

    println!("Searching for five-cliques...");
    let cliques = graph.search_for_clique();
    println!("Found {} five-cliques.", cliques.len());

    let output_filepath = "five_cliques.csv";
    write_cliques_to_file(output_filepath, &cliques).unwrap();
    println!("Wrote results to: {}", output_filepath);
}

type Word = String;
type WordIndex = usize;
type Words = Vec<(Word, BTreeSet<char>)>;
type Clique = BTreeSet<Word>;

type WordIndexSet = BitSet;

fn write_cliques_to_file(filepath: &str, cliques: &BTreeSet<Clique>) -> io::Result<()> {
    let mut output_file = File::create(filepath).unwrap();
    writeln!(
        output_file,
        "word_1,word_2,word_3,word_4,word_5,missing_letter"
    )?;
    for clique in cliques.iter() {
        for word in clique.iter() {
            write!(output_file, "{},", word)?;
        }

        let mut letters: Vec<char> = clique
            .iter()
            .flat_map(|word| word.chars().collect::<Vec<char>>())
            .collect();
        letters.sort();
        for l in 'a'..='z' {
            if !letters.contains(&l) {
                write!(output_file, "{}", l)?;
            }
        }
        writeln!(output_file)?;
    }

    Ok(())
}

struct Graph {
    edges: BTreeMap<WordIndex, WordIndexSet>,
    words: Vec<Word>,
}

impl Graph {
    fn build_from_words(words: &Words) -> Graph {
        let mut edges = BTreeMap::new();
        for (word_index, (word, letters)) in words.iter().enumerate().progress() {
            let mut neighbors: WordIndexSet = BitSet::with_capacity(words.len());
            for (word_2_index, (word_2, letters_2)) in words.iter().enumerate() {
                if word == word_2 {
                    continue;
                }

                if letters.intersection(&letters_2).next().is_none() {
                    neighbors.insert(word_2_index);
                }
            }

            edges.insert(word_index, neighbors);
        }

        Graph {
            edges,
            words: words.iter().map(|parts| parts.0.clone()).collect(),
        }
    }

    fn search_for_clique(&self) -> BTreeSet<Clique> {
        // We start with all the words as possible choices
        let neighbors_0: WordIndexSet = (0..self.words.len()).collect();

        // Parallelize the search over each starting word
        neighbors_0
            .iter()
            .collect::<Vec<WordIndex>>()
            .par_iter()
            .progress_count(neighbors_0.len() as u64)
            .flat_map(|word_index| self.search_for_clique_with_word(*word_index, &neighbors_0))
            .collect()
    }

    fn search_for_clique_with_word(
        &self,
        word_0: WordIndex,
        neighbors_0: &WordIndexSet,
    ) -> BTreeSet<Clique> {
        let mut solutions = BTreeSet::new();

        // Pre-allocate all the neighbor sets we will need. By allocate them once and
        // clearing + populating them as needed, we can save significantly on runtime
        let mut neighbors_1: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_2: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_3: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_4: WordIndexSet = BitSet::with_capacity(self.words.len());

        // For each of the words (second through fifth), narrow down the possibilities and try
        // each of them (skipping duplicate combinations).
        self.populate_new_possibilities(&neighbors_0, word_0, &mut neighbors_1);
        for word_1 in neighbors_1.iter() {
            if word_1 < word_0 {
                continue;
            }

            self.populate_new_possibilities(&neighbors_1, word_1, &mut neighbors_2);
            for word_2 in neighbors_2.iter() {
                if word_2 < word_1 {
                    continue;
                }

                self.populate_new_possibilities(&neighbors_2, word_2, &mut neighbors_3);
                for word_3 in neighbors_3.iter() {
                    if word_3 < word_2 {
                        continue;
                    }

                    self.populate_new_possibilities(&neighbors_3, word_3, &mut neighbors_4);
                    for word_4 in neighbors_4.iter() {
                        if word_4 < word_3 {
                            continue;
                        }

                        // Found a five-clique!
                        let current = Graph::words_to_set(&vec![
                            &self.words[word_0],
                            &self.words[word_1],
                            &self.words[word_2],
                            &self.words[word_3],
                            &self.words[word_4],
                        ]);

                        solutions.insert(current);
                    }
                }
            }
        }

        solutions
    }

    fn populate_new_possibilities(
        &self,
        previous_neighbors: &WordIndexSet,
        word_index: WordIndex,
        destination: &mut WordIndexSet,
    ) {
        // Copy the previous neighbors
        destination.clear();
        destination.union_with(previous_neighbors);

        // Intersect with the new neighbors
        let word_neighbors = self.edges.get(&word_index).unwrap();
        destination.intersect_with(word_neighbors);
    }

    fn words_to_set(words: &Vec<&Word>) -> BTreeSet<Word> {
        words.iter().cloned().cloned().collect()
    }
}

fn read_words_file(filepath: &str) -> io::Result<Words> {
    let mut words = vec![];
    let file = File::open(filepath)?;
    for word in io::BufReader::new(file).lines() {
        let word = word?;
        if word.len() == 5 {
            let letters: BTreeSet<char> = word.chars().collect();
            if letters.len() == 5 {
                words.push((word, letters));
            }
        }
    }

    Ok(words)
}

fn calculate_letter_frequencies(words: &Words) -> BTreeMap<char, i64> {
    let mut frquencies: BTreeMap<char, i64> = BTreeMap::new();
    for (_, letters) in words.iter() {
        for letter in letters.iter() {
            frquencies
                .entry(*letter)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    frquencies
}
