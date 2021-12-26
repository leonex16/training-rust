// Minigrep - Chapter 12
use std::error::Error;
use std::{env, fs};

struct Config<'a> {
  query: &'a str,
  filename: &'a str,
  case_sensitive: bool,
}

impl<'a> Config<'a> {
  fn new<'b: 'a>(args: &'b Vec<String>) -> Result<Self, &str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    let query = args[1].as_str();
    let filename = args[2].as_str();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Self {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn minigrep() {
  let args: Vec<String> = std::env::args().collect();
  let config: Config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);
    std::process::exit(1);
  };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  if config.case_sensitive {
    if let Err(_) = search_case_insensitive(config) {
      eprint!("Something went wrong during the process");
      std::process::exit(1);
    }
  } else {
    if let Err(_) = search(config) {
      eprint!("Something went wrong during the process");
      std::process::exit(1);
    }
  }

  Ok(())
}

fn search(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

  for line in content.lines() {
    if line.contains(config.query) == false {
      continue;
    }

    println!("{}", line.trim());
  }

  Ok(())
}

fn search_case_insensitive(config: Config) -> Result<(), Box<dyn Error>> {
  let query = config.query.to_lowercase();
  let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

  for line in content.lines() {
    if line.to_lowercase().contains(&query) == false {
      continue;
    }

    println!("{}", line.trim())
  }

  Ok(())
}

#[cfg(test)]
mod test {

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
      Rust:
      safe, fast, productive.
      Pick three.
      Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
      Rust:
      safe, fast, productive.
      Pick three.
      Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }

  fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
      if line.contains(query) == false {
        continue;
      }
      results.push(line.trim());
    }

    results
  }

  fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
        results.push(line.trim());
      }
    }

    results
  }
}
