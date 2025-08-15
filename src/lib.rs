use std::error::Error;
use std::{env, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let search_results = if !config.ignore {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    print_results(&config.query, &search_results);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: match args.next() {
                None => return Err("缺少搜索关键字参数"),
                Some(x) => x,
            },
            file_path: match args.next() {
                None => return Err("缺少文件路径参数"),
                Some(x) => x,
            },
            ignore: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_ascii_lowercase();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_ascii_lowercase().contains(&query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

fn print_results(query: &str, results: &[(usize, &str)]) {
    if results.is_empty() {
        println!(" 未找到包含 '{}' 的匹配项", query);
        return;
    }

    let total_matches = results.len();

    println!("------------------------");
    println!("! 搜索: \"{query}\" 共找到 {} 处匹配", total_matches);
    println!("------------------------");

    for (line_num, line) in results {
        let highlighted_line = line.replace(query, &format!("\x1b[1;31m{query}\x1b[0m"));

        println!(" {:>4} │ {}", line_num, highlighted_line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "jjj";
        let content = "\
dasdas
dfsgsdfgtert
asdasjjjaaaaa
dadsadasdsa
asdsajjjadasddddddd
.";

        let results = search(query, content);
        let expected: Vec<(usize, &str)> = vec![(3, "asdasjjjaaaaa"), (5, "asdsajjjadasddddddd")];

        assert_eq!(results, expected);
    }

    #[test]
    fn case_insensitive() {
        let query = "JJJ";
        let content = "\
dasdas
dfsgsdfgtert
asdasJJJaaaaa
dadsadasdsa
asdsajjjadasddddddd
.";

        let results = search_case_insensitive(query, content);
        let expected: Vec<(usize, &str)> = vec![(3, "asdasJJJaaaaa"), (5, "asdsajjjadasddddddd")];

        assert_eq!(results, expected);
    }
}
