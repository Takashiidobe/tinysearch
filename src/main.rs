use std::{
    collections::{BTreeSet, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
};

use patricia_tree::PatriciaMap;

use anyhow::Result;
// use glob::glob;

fn main() -> Result<()> {
    // let mut mytrie = PatriciaMap::default();

    // for entry in glob("./data/**/*.md")? {
    //     let path = entry?;
    //     let path = path.to_string_lossy().to_string();

    //     let f = File::open(&*path)?;
    //     let f = BufReader::new(f);

    //     for line in f.lines() {
    //         let line = line?;
    //         let s: String = line.to_string();
    //         let words: Vec<String> = s.split_whitespace().map(|s| s.to_owned()).collect();
    //         let word_windows: Vec<String> = words.windows(5).map(|w| w.join(" ")).collect();
    //         for fivegram in &word_windows {
    //             if !mytrie.contains_key(fivegram) {
    //                 let mut set = BTreeSet::default();
    //                 set.insert(path.clone());
    //                 mytrie.insert(fivegram, set);
    //             } else {
    //                 let mut set: BTreeSet<String> = mytrie.get(fivegram).unwrap().to_owned();
    //                 set.insert(path.clone());
    //                 mytrie.insert(fivegram, set);
    //             }
    //         }
    //     }
    // }
    use std::env::args;
    match args().len() {
        0..=1 => eprintln!("Please provide at least one word to query with"),
        _ => {
            let search_words: Vec<_> = args().collect();
            let needle: Vec<u8> = {
                let joined = search_words[1..].join(" ");
                joined.bytes().collect()
            };

            // let encoded = bincode::serialize(&mytrie)?;
            // fs::write("./data.index", encoded)?;
            let index_file = File::open("./data.index")?;
            let decoded: PatriciaMap<BTreeSet<String>> = bincode::deserialize_from(index_file)?;

            let matches: Vec<_> = decoded.iter_prefix(&needle).collect();

            // just pick one match
            let mut paths: HashSet<_> = HashSet::default();
            for (_key, val) in &matches {
                paths.extend(val.iter());
            }

            for (key, val) in &matches {
                let key = String::from_utf8_lossy(key);
                dbg!(key, val);
            }

            println!("{:#?}", paths);
        }
    }

    Ok(())
}
