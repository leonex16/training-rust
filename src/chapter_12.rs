// Minigrep - Chapter 12
use std::fs;
use std::error::Error;

struct Config<'a> {
  query: &'a str,
  filename: &'a str,
}

impl<'a> Config<'a> {
  fn new<'b: 'a>(args: &'b Vec<String>) -> Result<Self, &str> {
    if args.len() < 3 { return Err("Not enough arguments"); }
    let query = args[1].as_str();
    let filename = args[2].as_str();

    Ok(Self { query, filename })
  }
}

pub fn minigrep() {
  let args: Vec<String> = std::env::args().collect();
  let config: Config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });

  if let Err(e) = run(config) {
    println!("Application error: {}", e);

    std::process::exit(1);
  };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

  println!("With text:\n{}", content);

  Ok(())
}

fn search( query: &str, content: &str ) -> Vec<&str> {
    vect![]
}

#[cfg(test)]
mod tests {
    use super::search;

    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}