extern crate rand;

use std::{
    env, 
    fs::File,
    io::Read,
    collections::HashMap,
};


fn main() {
    // Get the text from the passed filepath.
    let text = {
        let mut text = String::new();
        File::open(env::args().nth(1).unwrap()).unwrap()
            .read_to_string(&mut text).unwrap();
        text
    };

    // Generate statistics
    let mut stats: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    let mut word_itr = text.split_whitespace();
    let mut prev_word = word_itr.next().unwrap();
    let mut word_count = 1;
    for word in word_itr {
        stats.entry(prev_word).or_insert(HashMap::new())
            .entry(word).and_modify(|e| *e += 1).or_insert(1);
        prev_word = word;
        word_count += 1;
    };

    // Generate the text
    let mut word = text.split_whitespace().nth(rand::random::<usize>() % word_count).unwrap();
    for _ in 0..100 {
        print!("{} ", word);
        let freq_sum: usize = stats[word].values().sum();
        let n = rand::random::<usize>() % freq_sum;
        let mut i = 0;
        for (new_word, freq) in stats[word].iter() {
            i += freq;
            if i > n {
                word = new_word;
                break;
            }
        }; 
    };
    println!("");
}
