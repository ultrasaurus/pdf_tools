use clap::Parser;
use pdf::file::File;
use pdf_tools::page_text;
use rust_bert::pipelines::pos_tagging::POSModel;

#[derive(Parser)]
struct Cli {
    /// PDF input file
    #[clap(short, long, value_parser, default_value = "../hello-world.pdf")]
    input: String,
}

fn tag() -> () {
    //    Set-up model
    let pos_model = POSModel::new(Default::default()).expect("default POSModel");

    //    Define input
    let input = ["Bob is arrogant. Maria is confident."];

    //    Run model
    let output = pos_model.predict(&input);
    for (pos, pos_tag) in output[0].iter().enumerate() {
        println!("{} - {:?}", pos, pos_tag);
    }
    {}
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let file = File::open(&input).expect("failed to read PDF");
    tag();
    for (page_num, page) in file.pages().enumerate() {
        if let Ok(page) = page {
            println!("=== PAGE {} ===\n", page_num + 1);
            if let Ok(text) = page_text(&page, &file) {
                println!("{}", text);
            } else {
                println!("ERROR");
            }
            println!();
        }
    }
}
