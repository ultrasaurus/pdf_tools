use clap::Parser;
use pdf::file::File;
use pdf_tools::page_text;

#[derive(Parser)]
struct Cli {
    /// PDF input file
    #[clap(short, long, value_parser, default_value = "../hello-world.pdf")]
    input: String,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let file = File::open(&input).expect("failed to read PDF");
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
