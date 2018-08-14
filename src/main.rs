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
    for word in word_itr {
        let e = stats.entry(prev_word).or_insert(HashMap::new()).entry(word).or_insert(0);
        *e += 1;
        prev_word = word;
    };
    
    println!("{}", text);
}
