use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    io::BufRead,
};

use indicatif::ProgressIterator;

fn main() {
    println!("Reading in words file...");
    let words = read_words_file("data/words_alpha.txt").unwrap();
    println!("words.len() = {}", words.len());

    println!("Building up the graph...");
    let graph = Graph::build_from_words(&words);
    println!("Counting the number of edges...");
    let num_edges: usize = words
        .iter()
        .map(|word| graph.edges.get(&word.0).unwrap().len())
        .sum();
    println!("num_edges = {}", num_edges);

    let clique = graph.search_for_clique(5);
    if let Some(c) = clique {
        println!("Found a clique:");
        for word in c.iter() {
            println!("{}", word);
        }

        println!("\nletters:");
        let mut letters: Vec<char> = c
            .iter()
            .flat_map(|word| word.chars().collect::<Vec<char>>())
            .collect();
        letters.sort();
        for l in letters.iter() {
            print!("{}", l);
        }
        println!();
    } else {
        println!("Did not find a clique.");
    }
}

type Word = String;
type Words = Vec<(Word, BTreeSet<char>)>;

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

    fn search_for_clique(&self, clique_size: usize) -> Option<BTreeSet<Word>> {
        self.search_for_clique_inner(
            clique_size,
            BTreeSet::new(),
            self.edges.keys().cloned().collect(),
        )
    }

    fn search_for_clique_inner(
        &self,
        clique_size: usize,
        existing_members: BTreeSet<Word>,
        neighbors: BTreeSet<Word>,
    ) -> Option<BTreeSet<Word>> {
        if clique_size == 0 {
            return Some(existing_members);
        }

        for neighbor in neighbors.iter() {
            let mut new_members = existing_members.clone();
            new_members.insert(neighbor.clone());

            let new_neighbors = neighbors
                .intersection(self.edges.get(neighbor).unwrap())
                .cloned()
                .collect();

            let solution =
                self.search_for_clique_inner(clique_size - 1, new_members, new_neighbors);
            if solution.is_some() {
                return solution;
            }
        }

        return None;
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
