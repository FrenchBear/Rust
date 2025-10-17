// rcheckfiles tests
//
// 2025-04-08   PV
// 2025-10-16	PV      Complete set of tests for check_basename
// 2025-10-17	PV      test_check_basename_characters_to_remove

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
    use crate::Statistics;
    use crate::check_basename;
    use logging::logwriter_none;
    use std::path::Path;
    use std::sync::LazyLock;

    struct SharedData {
        transformation_data: crate::TransformationData,
        options: crate::Options,
    }

    static SHARED_DATA: LazyLock<SharedData> = LazyLock::new(|| SharedData {
        options: crate::Options::default(),
        transformation_data: crate::get_transformation_data(),
    });

    #[test]
    fn test_check_basename_non_normalized() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("État de siège à Katmandou"), // all 3 accents are not normalized
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.nnn, 1);
        assert_eq!(res.unwrap(), "État de siège à Katmandou");
    }

    #[test]
    fn test_check_basename_apostrophes() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("A\u{00B4}B\u{02B9}C\u{02BB}D\u{02BC}E\u{02BD}F\u{02BE}G"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.apo, 1);
        assert_eq!(res.unwrap(), "A'B'C'D'E'F'G");
    }

    #[test]
    fn test_check_basename_spaces() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("A\u{2006}B\u{2007}C\u{2002}D\u{2003}E\u{200A}F"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.spc, 1);
        assert_eq!(res.unwrap(), "A B C D E F");
    }

    #[test]
    fn test_check_basename_invalid_chars() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("Mahjong tiles 🀇🀈🀉🀊🀋🀌🀍🀎🀏 🀀🀁🀂🀃.docx"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_none());     // invalid chars are not automatically fixed, there is no fixed name
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.car, 1);
    }

        #[test]
    fn test_check_basename_ligatures() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("oﬃce ﬁle cœur ﬆar"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.lig, 1);
        assert_eq!(res.unwrap(), "office file coeur star");
    }

    #[test]
    fn test_check_basename_multiple_spaces() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("Il  était   \u{00A0}  \u{00A0}une\u{00A0}\u{00A0}fois"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            2
        );
        assert_eq!(files_stats.spc, 1);
        assert_eq!(files_stats.sp2, 1);
        assert_eq!(res.unwrap(), "Il était une fois");
    }

    #[test]
    fn test_check_basename_space_before() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("A !B ,C ¿D …[E  ]F(G )"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            6
        );
        assert_eq!(files_stats.sba, 6);
        assert_eq!(res.unwrap(), "A!B,C¿D…[E]F(G)");
    }

    #[test]
    fn test_check_basename_space_after() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("file with ( spaces  ) before [   and ] after « brackets »"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            6
        );
        assert_eq!(files_stats.sba, 6);
        assert_eq!(res.unwrap(), "file with (spaces) before [and] after «brackets»");
    }

    #[test]
    fn test_check_basename_characters_to_remove() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_basename(
            Path::new("A\u{FEFF}\u{FEFF}B\u{200E}C"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data, true
        );
        assert!(res.is_some());
        assert_eq!(
            files_stats.nnn
                + files_stats.bra
                + files_stats.apo
                + files_stats.spc
                + files_stats.car
                + files_stats.sp2
                + files_stats.lig
                + files_stats.sba,
            1
        );
        assert_eq!(files_stats.car, 1);
        assert_eq!(res.unwrap(), "ABC");
    }
}
