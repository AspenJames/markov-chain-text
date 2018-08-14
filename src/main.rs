extern crate rand;

use std::{
    env, 
    fs::File,
    io::Read,
};


fn main() {
    // Get the text from the passed filepath.
    let text = {
        let mut text = String::new();
        File::open(env::args().nth(1).unwrap()).unwrap()
            .read_to_string(&mut text).unwrap();
        text
    };
    
    println!("{}", text);
}
