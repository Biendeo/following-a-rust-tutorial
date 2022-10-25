use std::env;
use std::error::Error;
use std::fs;

// Pub on all the fields so they can be publicly accessed. This could be fansified with getter/setters.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // This was named "build" because "new" sounds like it won't fail whereas "build" might if you pass in wrong things.
    // &'static means that the string has a static lifetime (meaning it lasts the lifetime of the program, and it's readonly). "'static" with the apostrophe is the keyword here.
    // https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // This is < 3 because the program name is one argument.
        if args.len() < 3 {
            // We're throwing Err here because calling panic! is not as easy to work with.
            return Err("not enough arguments");
        }

        // These are cloned so the strings aren't either moved or referenced.
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Environment variable grab.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Struct construction (wrapped in an OK as well).
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // This reads the whole file contents as one string.
    let contents = fs::read_to_string(config.file_path)?;

    // This is a function pointer, the type uses this fancy "for" so that the lifetime specifier can be used.
    let search_method: for<'a> fn(&str, &'a str) -> Vec<&'a str> = if config.ignore_case {
        search
    } else {
        search_case_insensitive
    };

    let results = search_method(&config.query, &contents);

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// The <'a> is here to indicate that something has an explicit lifetime elisium.
// https://doc.rust-lang.org/1.6.0/book/lifetimes.html#lifetime-elision
// Short terms, it's basically indicating that the strings in the result should have the same lifetimes as the string in contents (since it's technically made of splits).
// This would not be needed if query was hardcoded and didn't need to be an argument.
// Otherwise you get "help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`".
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // This expands out the string's lines (similar to Python's .splitlines())
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}