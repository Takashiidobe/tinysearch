use std::{
    collections::{BTreeSet, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
};

use patricia_tree::PatriciaMap;

use anyhow::Result;
use glob::glob;

fn index(pattern: &str) -> Result<()> {
    let mut mytrie = PatriciaMap::default();

    for entry in glob(pattern)? {
        let path = entry?;
        let path = path.to_string_lossy().to_string();

        let f = File::open(&*path)?;
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line?;
            let s: String = line.to_string();
            let words: Vec<String> = s.split_whitespace().map(|s| s.to_owned()).collect();
            let word_windows: Vec<String> = words.windows(5).map(|w| w.join(" ")).collect();
            for fivegram in &word_windows {
                if !mytrie.contains_key(fivegram) {
                    let mut set = BTreeSet::default();
                    set.insert(path.clone());
                    mytrie.insert(fivegram, set);
                } else {
                    // we know that the trie contains fivegram, so unwrapping is safe.
                    let mut set: BTreeSet<String> = mytrie.get(fivegram).unwrap().to_owned();
                    set.insert(path.clone());
                    mytrie.insert(fivegram, set);
                }
            }
        }
    }
    let encoded = bincode::serialize(&mytrie)?;
    fs::write("./data.index", encoded)?;
    Ok(())
}

fn search(search_words: &[String]) -> Result<()> {
    let needle: Vec<u8> = {
        let joined = search_words.join(" ");
        joined.bytes().collect()
    };

    let index_file = File::open("./data.index")?;
    let decoded: PatriciaMap<BTreeSet<String>> = bincode::deserialize_from(index_file)?;

    let matches: Vec<_> = decoded.iter_prefix(&needle).collect();

    let mut paths: HashSet<_> = HashSet::default();
    for (_key, val) in &matches {
        paths.extend(val.iter());
    }

    println!("{:#?}", paths);
    Ok(())
}

fn main() -> Result<()> {
    use std::env::args;
    match args().len() {
        0 => eprintln!("Please provide a top level command of search or index"),
        _ => {
            let arguments: Vec<_> = args().collect();
            if args().len() > 2 && arguments[1] == "index" {
                index(&arguments[2]);
            } else if args().len() > 2 && arguments[1] == "search" {
                search(&arguments[2..]);
            } else {
                eprintln!("Please provide a top level command of search or index");
            }
        }
    }
    Ok(())
}
