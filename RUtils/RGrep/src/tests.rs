// rgrep tests
//
// 2025-03-14   PV
// 2025-04-01   PV      Adapted tests to read_text_file_2
// 2025-05-02   PV      Removed decode_encoding tests, moved to crate TextAutoDecode

#[cfg(test)]

#[cfg(test)]
pub mod grep_iterator {
    use crate::grepiterator::GrepLineMatches;
    use regex::Regex;

    #[test]
    fn iterator() {
        let re = Regex::new("(?imR)pommes").unwrap();
        let haystack = "RECETTE DE LA TARTE AUX POMMES\r\nPréparer la pâte\r\nPrécuire la pâte 10 minutes\r\nPeler les pommes et ajouter les pommes\r\nFaire cuire\r\nVous préférez froid? Laisser refroidir\r\nDéguster!";

        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].line, "RECETTE DE LA TARTE AUX POMMES");
        assert_eq!(res[0].ranges.len(), 1);
        assert_eq!(res[0].ranges[0], 24..30);

        assert_eq!(res[1].line, "Peler les pommes et ajouter les pommes");
        assert_eq!(res[1].ranges.len(), 2);
        assert_eq!(res[1].ranges[0], 10..16);
        assert_eq!(res[1].ranges[1], 32..38);
    }
}

#[cfg(test)]
pub mod build_re {
    use crate::grepiterator::GrepLineMatches;
    use crate::{Options, build_re};

    #[test]
    fn case_sensitive() {
        let haystack = "RECETTE DE LA TARTE AUX POMMES\r\nPréparer la pâte\r\nPrécuire la pâte 10 minutes\r\nPeler les pommes et ajouter les pommes\r\nFaire cuire\r\nVous préférez froid? Laisser refroidir\r\nDéguster!";
        let options = Options {
            pattern: String::from("pommes"),
            ignore_case: false,
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn case_insensitive() {
        let haystack = "RECETTE DE LA TARTE AUX POMMES\r\nPréparer la pâte\r\nPrécuire la pâte 10 minutes\r\nPeler les pommes et ajouter les pommes\r\nFaire cuire\r\nVous préférez froid? Laisser refroidir\r\nDéguster!";
        let options = Options {
            pattern: String::from("pommes"),
            ignore_case: true,
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn full_line() {
        let haystack = "RECETTE DE LA TARTE AUX POMMES\r\nPréparer la pâte\r\nPrécuire la pâte 10 minutes\r\nPeler les pommes et ajouter les pommes\r\nFaire cuire\r\nVous préférez froid? Laisser refroidir\r\nDéguster!";
        let options = Options {
            pattern: String::from("^Préparer la pâte$"),
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn fixed_string() {
        let haystack = "RECETTE DE LA TARTE AUX POMMES\r\nPréparer la pâte\r\nPrécuire la pâte 10 minutes\r\nPeler les pommes et ajouter les pommes\r\nFaire cuire\r\nVous préférez froid? Laisser refroidir\r\nDéguster!";
        let options = Options {
            pattern: String::from("froid? Laisser"),
            fixed_string: true,
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn special_question_mark() {
        let haystack = "Astérix et Obélix?\r\nTom [et] Jerry\r\nLaurel*Hardy\r\n";
        let options = Options {
            pattern: String::from(r"[?]"),
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn special_question_bracket() {
        let haystack = "Astérix et Obélix?\r\nTom [et] Jerry\r\nLaurel*Hardy\r\n";
        let options = Options {
            pattern: String::from(r"[\[]"),
            ..Default::default()
        };
        let re = build_re(&options).unwrap();
        let res: Vec<GrepLineMatches> = GrepLineMatches::new(haystack, &re).collect();
        assert_eq!(res.len(), 1);
    }

}
