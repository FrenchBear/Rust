// rcheckfiles tests
//
// 2025-04-08   PV
// 2025-10-16   PV      Added test_check_basename, for now just a single test

#[cfg(test)]
pub mod balanced_tests {
    use crate::is_balanced;

    #[test]
    fn balanced1() {
        assert!(is_balanced("simple texte"));
    }

    #[test]
    fn balanced2() {
        assert!(is_balanced("a (b) [c] {d} «e» ‹f›"));
    }

    #[test]
    fn balanced3() {
        assert!(is_balanced("({[«‹Hello›»]})"));
    }

    #[test]
    fn balanced4() {
        assert!(is_balanced("((a[[b]c]d)e)"));
    }

    #[test]
    fn balanced5() {
        assert!(is_balanced(""));
    }

    #[test]
    fn not_balanced1() {
        assert!(!is_balanced("((a((b)cd)e)"));
    }

    #[test]
    fn not_balanced2() {
        assert!(!is_balanced("a(b[c]}"));
    }

    #[test]
    fn not_balanced3() {
        assert!(!is_balanced("pom)me"));
    }
}

#[cfg(test)]
pub mod check_basename_tests {
    use std::path::Path;
    use logging::logwriter_none;
    use crate::check_basename;
    use crate::Statistics;

    #[test]
    fn test_check_basename() {
        let mut files_stats = Statistics { ..Default::default() };
        let options = crate::Options::default();
        let transformation_data = crate::get_transformation_data();

        let res = check_basename(Path::new("file with ( spaces  ) before [   and ] after « brackets »"), "file", &mut files_stats, &options, &mut logwriter_none(), &transformation_data);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), "file with (spaces) before [and] after «brackets»");
    }
}
