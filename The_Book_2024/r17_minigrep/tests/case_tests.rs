// More tests for mnigrep, case-based tests
//
// 2014-12-10   PV

#[cfg(test)]
mod tests {

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            r17_minigrep::search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            r17_minigrep::search_case_insensitive(query, contents)
        );
    }
}
