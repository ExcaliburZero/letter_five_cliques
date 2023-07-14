use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    io::BufRead,
};

use indicatif::ProgressBar;
use indicatif::ProgressIterator;

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
    println!("Counting the number of edges...");
    let num_edges: usize = words
        .iter()
        .map(|word| graph.edges.get(&word.0).unwrap().len())
        .sum();
    println!("num_edges = {}", num_edges);

    let cliques = graph.search_for_clique(5);
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
type Words = Vec<(Word, BTreeSet<char>)>;
type Clique = BTreeSet<Word>;

struct Graph {
    edges: BTreeMap<Word, BTreeSet<Word>>,
}

impl Graph {
    fn build_from_words(words: &Words) -> Graph {
        let mut edges = BTreeMap::new();
        for (word, letters) in words.iter().progress() {
            let mut neighbors = BTreeSet::new();
            for (word_2, letters_2) in words.iter() {
                if word == word_2 {
                    continue;
                }

                if letters.intersection(&letters_2).next().is_none() {
                    neighbors.insert(word_2.clone());
                }
            }

            edges.insert(word.clone(), neighbors);
        }

        Graph { edges }
    }

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
