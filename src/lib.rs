pub fn search<'a>(query: &'a str, contents: &'a str, ignore_case: bool) 
-> impl Iterator<Item = &'a str> {
    let query_lower = query.to_lowercase();

    contents
        .lines()
        .filter(move |line| {
            if ignore_case {
                line.to_lowercase().contains(&query_lower)
            } else {
                line.contains(query)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results: Vec<_> =  search(query, contents, false).collect();

        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results: Vec<_> = search(query, contents, true).collect();

        assert_eq!(vec!["Rust:", "Trust me."], results);
    }
}