// rcheckfiles tests
//
// 2025-04-08   PV
// 2025-10-16	PV      Complete set of tests for check_basename
// 2025-10-17	PV      test_check_basename_characters_to_remove
// 2025-10-18	PV      test_check_basename_ends_with_one/three/four_dots
// 2025-10-21	PV      test_double_extension
// 2025-10-24	PV      test_check_basename_dashes

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
    use crate::check_name;
    use crate::is_single_script;
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

    fn get_sum(files_stats: &Statistics) -> u32 {
        files_stats.nnn
            + files_stats.bra
            + files_stats.spc
            + files_stats.apo
            + files_stats.das
            + files_stats.car
            + files_stats.sp2
            + files_stats.lig
            + files_stats.sba
            + files_stats.ewd
            + files_stats.dex
            + files_stats.usd
    }

    #[test]
    fn test_check_basename_non_normalized() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("État de siège à Katmandou"), // all 3 accents are not normalized
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.nnn, 1);
        assert_eq!(res.unwrap(), "État de siège à Katmandou");
    }

    #[test]
    fn test_check_basename_spaces() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("A\u{2006}B\u{2007}C\u{2002}D\u{2003}E\u{200A}F"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.spc, 1);
        assert_eq!(res.unwrap(), "A B C D E F");
    }

    #[test]
    fn test_check_basename_apostrophes() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("A-B˗C۔D‐E‑F‒G–H⁃I−J"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.das, 1);
        assert_eq!(res.unwrap(), "A-B-C-D-E-F-G-H-I-J");
    }

    #[test]
    fn test_check_basename_dashes() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("A\u{2006}B\u{2007}C\u{2002}D\u{2003}E\u{200A}F"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.spc, 1);
        assert_eq!(res.unwrap(), "A B C D E F");
    }

    #[test]
    fn test_check_basename_invalid_chars() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Mahjong tiles 🀇🀈🀉🀊🀋🀌🀍🀎🀏 🀀🀁🀂🀃.docx"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_none()); // invalid chars are not automatically fixed, there is no fixed name
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.car, 1);
    }

    #[test]
    fn test_check_basename_ligatures() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("oﬃce ﬁle cœur ﬆar"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.lig, 1);
        assert_eq!(res.unwrap(), "office file coeur star");
    }

    #[test]
    fn test_check_basename_multiple_spaces() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Il  était   \u{00A0}  \u{00A0}une\u{00A0}\u{00A0}fois"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 2);
        assert_eq!(files_stats.spc, 1);
        assert_eq!(files_stats.sp2, 1);
        assert_eq!(res.unwrap(), "Il était une fois");
    }

    #[test]
    fn test_check_basename_space_before() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("A !B ,C ¿D …[E  ]F(G )"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 6);
        assert_eq!(files_stats.sba, 6);
        assert_eq!(res.unwrap(), "A!B,C¿D…[E]F(G)");
    }

    #[test]
    fn test_check_basename_space_after() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("file with ( spaces  ) before [   and ] after « brackets »"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 6);
        assert_eq!(files_stats.sba, 6);
        assert_eq!(res.unwrap(), "file with (spaces) before [and] after «brackets»");
    }

    #[test]
    fn test_check_basename_characters_to_remove() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("A\u{FEFF}\u{FEFF}B\u{200E}C"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.car, 1);
        assert_eq!(res.unwrap(), "ABC");
    }

    #[test]
    fn test_check_basename_ends_with_three_dots() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Once upon a time....txt"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.ewd, 1);
        assert_eq!(res.unwrap(), "Once upon a time….txt");
    }

    #[test]
    fn test_check_basename_ends_with_one_dot() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Once upon a time..doc"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_none()); // Not fixed
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.ewd, 1);
    }

    #[test]
    fn test_check_basename_ends_with_four_dots() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Once upon a time.....docx"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_none()); // Not fixed
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.ewd, 1);
    }

    #[test]
    fn test_check_basename_double_extension() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("My document.pdf.pdf"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_some());
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.dex, 1);
        assert_eq!(res.unwrap(), "My document.pdf");
    }

    #[test]
    fn test_check_basename_mixed_scripts() {
        assert_eq!(is_single_script("ABCabc ΑΒΓαβγ АБВабв"), true);
        assert_eq!(is_single_script("ABCabc AΒΓαβγ АБВабв"), false); // 2nd A is latin, not greek
        assert_eq!(is_single_script("ABCabcΑΒΓαβγАБВабв"), false);
        assert_eq!(is_single_script("Ƥḭҽɾɾҽ ѵìǫłҽղէ.docx"), false);
        assert_eq!(is_single_script("2πr πr² Δt Ωmega"), true); // πΔΩ can be mixed with other languages
    }

    #[test]
    fn test_check_basename_unbalanced_spaces_dashes() {
        let mut files_stats = Statistics { ..Default::default() };

        let res = check_name(
            Path::new("Name -problem- end"),
            "file",
            &mut files_stats,
            &SHARED_DATA.options,
            &mut logwriter_none(),
            &SHARED_DATA.transformation_data,
            true,
        );
        assert!(res.is_none()); // Not fixed
        assert_eq!(get_sum(&files_stats), 1);
        assert_eq!(files_stats.usd, 1);
    }
}
