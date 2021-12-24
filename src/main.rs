use std::fs;

mod chapter_2;

fn main() {
    println!("*******************************************");
    println!("");
    // ****************************************************
    
    //chapter_2::guessing_game();

    let args: Vec<String> = std::env::args().collect();
    let Config { query, filename} = Config::new(&args);

    let content = fs::read_to_string(filename);
    println!("{:#?}", query);
    println!("{:#?}", content);
    // ****************************************************
    println!("");
    println!("*******************************************");
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str
}

impl<'a> Config<'a> {
    fn new<'b: 'a>(args: &'b Vec<String>) -> Self {
        let query = args[1].as_str();
        let filename = args[2].as_str();
    
        Config { query, filename }
    }
}