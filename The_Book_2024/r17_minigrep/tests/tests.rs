// unit tests for r17_minigrep
// Learning rust
//
// 2024-12-08   PV      Structure different than what is in the book; tests in a separate file

#[cfg(test)]
pub mod tests {
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], r17_minigrep::search(query, contents));
    }    
}
