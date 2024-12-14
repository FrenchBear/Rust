// vstring unit tests - Glyph tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod glyph_tests {
    use crate::glyph::Glyph;

    #[test]
    fn glyph_tests_simple() {
        let s = "AB";
        let v = Glyph::glyph_indices(s).collect::<Vec<(usize, Glyph)>>();

        assert!(v.len()==2);
        assert!(v[0].0==0);
        assert!(v[0].1.chars=="A");
        assert!(v[1].0==1);
        assert!(v[1].1.chars=="B");
    }

    #[test]
    fn glyph_tests_combining_accent() {
        let s = "ae\u{0301}z";
        let v = Glyph::glyph_indices(s).collect::<Vec<(usize, Glyph)>>();

        assert!(v.len()==3);
        assert!(v[0].0==0);
        assert!(v[0].1.chars=="a");
        assert!(v[1].0==1);
        assert!(v[1].1.chars=="e\u{0301}");
        assert!(v[2].0==3);
        assert!(v[2].1.chars=="z");
    }

    #[test]
    fn glyph_tests_empty() {
        let v = Glyph::glyph_indices("").collect::<Vec<(usize, Glyph)>>();
        assert!(v.len()==0);
    }

}
