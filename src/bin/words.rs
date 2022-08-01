use clap::Parser;
use pdf::file::File;
use pdf_tools::page_text;
use rust_bert::pipelines::pos_tagging::POSModel;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    /// PDF input file
    #[clap(short, long, value_parser, default_value = "../hello-world.pdf")]
    input: String,
}

struct WordMap {
    map: HashMap<String, usize>,
}

impl WordMap {
    pub fn new() -> Self {
        WordMap {
            map: HashMap::new(),
        }
    }

    // rust-bert Parts of Speech tagging labels may be this:
    // https://cs.nyu.edu/~grishman/jet/guide/PennPOS.html
    // 	7.	JJ	Adjective
    //  8.	JJR	Adjective, comparative
    //  9.	JJS	Adjective, superlative
    fn is_adjective(label: &str) -> bool {
        match label {
            "JJ" => true,
            "JJR" => true,
            "JJS" => true,
            _ => false,
        }
    }
    fn add_word(&mut self, word: &str) {
        let key = String::from(word);
        *self.map.entry(key).or_insert(0) += 1;
    }

    pub fn tag(&mut self, input_str: &str) {
        //    Set-up model
        let pos_model = POSModel::new(Default::default()).expect("default POSModel");

        //let input = ["Bob is arrogant. Maria is confident."];
        let input = [input_str];

        //    Run model
        let output = pos_model.predict(&input);
        for (pos, pos_tag) in output[0].iter().enumerate() {
            if WordMap::is_adjective(&pos_tag.label) {
                self.add_word(&pos_tag.word);
            }
            println!("{} - {:?}", pos, pos_tag);
        }
        {}
    }

    pub fn print(self) {
        if self.map.keys().count() == 0 {
            println!("no adjectives found");
        }
        for (k, v) in self.map {
            println!("{}\t{}", v, k);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let mut words = WordMap::new();

    let file = File::open(&input).expect("failed to read PDF");
    for (page_num, page) in file.pages().enumerate() {
        if let Ok(page) = page {
            println!("=== PAGE {} ===\n", page_num + 1);
            if let Ok(text) = page_text(&page, &file) {
                println!("{}", text);
                words.tag(&text);
            } else {
                println!("ERROR");
            }
            println!();
        }
    }
    words.print();
}
