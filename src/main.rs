use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    io::BufRead,
};

use bit_set::BitSet;
use indicatif::ProgressIterator;

fn main() {
    println!("Reading in words file...");
    let words = read_words_file("data/wordle-all.txt").unwrap();
    //let words = read_words_file("data/words_alpha.txt").unwrap();
    println!("words.len() = {}", words.len());

    let letter_frequencies = calculate_letter_frequencies(&words);
    for (letter, count) in letter_frequencies.iter() {
        println!("{} = {}", letter, count)
    }

    println!("Building up the graph...");
    let graph = Graph::build_from_words(&words);
    println!("Counting the number of edges...");
    /*let num_edges: usize = words
        .iter()
        .map(|word| graph.edges.get(&word.0).unwrap().len())
        .sum();
    println!("num_edges = {}", num_edges);*/

    let cliques = graph.search_for_clique_non_recursive();
    for clique in cliques.iter() {
        println!("Found a clique:");
        for word in clique.iter() {
            println!("{}", word);
        }

        println!("\nletters:");
        let mut letters: Vec<char> = clique
            .iter()
            .flat_map(|word| word.chars().collect::<Vec<char>>())
            .collect();
        letters.sort();
        for l in letters.iter() {
            print!("{}", l);
        }
        println!();
    }
}

type Word = String;
type WordIndex = usize;
type Words = Vec<(Word, BTreeSet<char>)>;
type Clique = BTreeSet<Word>;

type WordIndexSet = BitSet;

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

    fn search_for_clique_non_recursive(&self) -> BTreeSet<Clique> {
        let mut solutions = BTreeSet::new();

        // We start with all the words as possible choices
        let neighbors_0: WordIndexSet = (0..self.words.len()).collect();

        // Pre-allocate all the neighbor sets we will need. By allocate them once and
        // clearing + populating them as needed, we can save significantly on runtime
        let mut neighbors_1: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_2: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_3: WordIndexSet = BitSet::with_capacity(self.words.len());
        let mut neighbors_4: WordIndexSet = BitSet::with_capacity(self.words.len());

        for word_0 in neighbors_0.iter().collect::<Vec<usize>>().iter().progress() {
            neighbors_1.clear();
            neighbors_1.union_with(&neighbors_0);
            self.intersection_(&mut neighbors_1, *word_0);
            for word_1 in neighbors_1.iter() {
                if word_1 < *word_0 {
                    continue;
                }

                neighbors_2.clear();
                neighbors_2.union_with(&neighbors_1);
                self.intersection_(&mut neighbors_2, word_1);
                for word_2 in neighbors_2.iter() {
                    if word_2 < word_1 {
                        continue;
                    }

                    neighbors_3.clear();
                    neighbors_3.union_with(&neighbors_2);
                    self.intersection_(&mut neighbors_3, word_2);
                    for word_3 in neighbors_3.iter() {
                        if word_3 < word_2 {
                            continue;
                        }

                        neighbors_4.clear();
                        neighbors_4.union_with(&neighbors_3);
                        self.intersection_(&mut neighbors_4, word_3);
                        for word_4 in neighbors_4.iter() {
                            if word_4 < word_3 {
                                continue;
                            }

                            let current = Graph::words_to_set(&vec![
                                &self.words[*word_0],
                                &self.words[word_1],
                                &self.words[word_2],
                                &self.words[word_3],
                                &self.words[word_4],
                            ]);
                            println!("{:?}", current);

                            solutions.insert(current);
                        }
                    }
                }
            }
        }

        solutions
    }

    fn intersection_(&self, neighbors: &mut WordIndexSet, word_index: WordIndex) {
        let word_neighbors = self.edges.get(&word_index).unwrap();
        neighbors.intersect_with(word_neighbors);
    }

    /*
    fn search_for_clique_non_recursive(&self) -> BTreeSet<Clique> {
        let mut visited: HashSet<WordIndexSet>  = HashSet::new();
        let mut solutions = BTreeSet::new();

        let neighbors_0: WordIndexSet = (0..self.words.len()).collect();
        for word_0 in neighbors_0.iter().progress() {
            let current = Graph::word_ids_to_set(&vec![word_0]);
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            let neighbors_1: WordIndexSet = self.intersection(&neighbors_0, *word_0);
            for word_1 in neighbors_1.iter() {
                let current = Graph::word_ids_to_set(&vec![word_0, word_1]);
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);

                let neighbors_2: WordIndexSet = self.intersection(&neighbors_1, *word_1);
                for word_2 in neighbors_2.iter() {
                    let current = Graph::word_ids_to_set(&vec![word_0, word_1, word_2]);
                    if visited.contains(&current) {
                        continue;
                    }
                    visited.insert(current);

                    let neighbors_3: WordIndexSet = self.intersection(&neighbors_2, *word_2);
                    for word_3 in neighbors_3.iter() {
                        let current = Graph::word_ids_to_set(&vec![word_0, word_1, word_2, word_3]);
                        if visited.contains(&current) {
                            continue;
                        }
                        visited.insert(current);

                        let neighbors_4: WordIndexSet = self.intersection(&neighbors_3, *word_3);
                        for word_4 in neighbors_4.iter() {
                            let current = Graph::words_to_set(&vec![
                                &self.words[*word_0],
                                &self.words[*word_1],
                                &self.words[*word_2],
                                &self.words[*word_3],
                                &self.words[*word_4],
                            ]);
                            println!("{:?}", current);

                            solutions.insert(current);
                        }
                    }
                }
            }
        }

        solutions
    }*/

    fn words_to_set(words: &Vec<&Word>) -> BTreeSet<Word> {
        words.iter().cloned().cloned().collect()
    }

    fn word_ids_to_set(word_indices: &Vec<&WordIndex>) -> WordIndexSet {
        word_indices.iter().cloned().cloned().collect()
    }

    //fn intersection(&self, neighbors: &WordIndexSet, word_index: WordIndex) -> WordIndexSet {
    //    neighbors.intersection(self.edges.get(&word_index).unwrap()).cloned().collect()
    //}

    /*

    fn search_for_clique(&self, clique_size: usize) -> BTreeSet<Clique> {
        let mut bar = ProgressBar::new(self.edges.len() as u64);
        let solutions = self.search_for_clique_inner(
            clique_size,
            &BTreeSet::new(),
            &self.edges.keys().cloned().collect(),
            &mut BTreeSet::new(),
            &mut bar,
        );
        bar.finish();

        solutions
    }

    fn search_for_clique_inner(
        &self,
        clique_size: usize,
        existing_members: &BTreeSet<Word>,
        neighbors: &BTreeSet<Word>,
        visited: &mut BTreeSet<Clique>,
        bar: &mut ProgressBar,
    ) -> BTreeSet<Clique> {
        if clique_size == 0 {
            println!("{:?}", existing_members);

            let mut solutions = BTreeSet::new();
            solutions.insert(existing_members.clone());

            return solutions;
        }

        let mut solutions = BTreeSet::new();
        for neighbor in neighbors.iter() {
            if clique_size == 5 {
                bar.inc(1);
                println!("{}", neighbor);
            }

            let mut new_members = existing_members.clone();
            new_members.insert(neighbor.clone());

            if visited.contains(&new_members) {
                continue;
            }

            visited.insert(new_members.clone());

            let new_neighbors = neighbors
                .intersection(self.edges.get(neighbor).unwrap())
                .cloned()
                .collect();

            let mut sub_solutions = self.search_for_clique_inner(
                clique_size - 1,
                &new_members,
                &new_neighbors,
                visited,
                bar,
            );
            solutions.append(&mut sub_solutions);
        }

        solutions
    }
    */
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
