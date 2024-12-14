// vstring unit tests - Test functions based on byte range indexes
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod byterange_tests {
    use crate::*;

    // ------------------------
    // get byte slice
    // Test all range variants

    #[test]
    pub fn test_byteslice() {
        assert_eq!(get_byteslice("Hello"), ['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]);
        assert_eq!(get_byteslice(""), []);
    }
    
    #[test]
    pub fn test_byteslice_from_byterange() {
        assert_eq!(get_byteslice_from_byterange("Hello", 1usize..3usize), ['e' as u8, 'l' as u8]);
        assert_eq!(get_byteslice_from_byterange("Hello", 2usize..2usize), []);
    }

    #[test]
    #[should_panic]
    pub fn test_byteslice_from_byterange_panic_invalid_range() {
        let _ = get_byteslice_from_byterange("Hello", 3usize..1usize);
    }

    #[test]
    #[should_panic]
    pub fn test_byteslice_from_byterange_panic_invalid_range_start() {
        let _ = get_byteslice_from_byterange("Hello", 10usize..12usize);
    }

    #[test]
    #[should_panic]
    pub fn test_byteslice_from_byterange_panic_invalid_range_end() {
        let _ = get_byteslice_from_byterange("Hello", 2usize..12usize);
    }


    #[test]
    pub fn test_bytesliceoption_from_byterange() {
        assert_eq!(get_bytesliceoption_from_byterange("Hello", 1usize..3usize), Some(&['e' as u8, 'l' as u8][..]));
        assert_eq!(get_bytesliceoption_from_byterange("Hello", 2usize..2usize), Some(&[][..]));
        assert_eq!(get_bytesliceoption_from_byterange("Hello", 3usize..1usize), None);
        assert_eq!(get_bytesliceoption_from_byterange("Hello", 10usize..12usize), None);
        assert_eq!(get_bytesliceoption_from_byterange("Hello", 2usize..12usize), None);
    }

    #[test]
    pub fn test_bytesliceresult_from_byterange() {
        assert_eq!(get_bytesliceresult_from_byterange("Hello", 1usize..3usize), Ok(&['e' as u8, 'l' as u8][..]));
        assert_eq!(get_bytesliceresult_from_byterange("Hello", 2usize..2usize), Ok(&[][..]));
        assert!(get_bytesliceresult_from_byterange("Hello", 3usize..1usize).is_err());
        assert!(get_bytesliceresult_from_byterange("Hello", 10usize..12usize).is_err());
        assert!(get_bytesliceresult_from_byterange("Hello", 2usize..12usize).is_err());
    }

    #[test]
    pub fn test_byteslicetolerant_from_byterange() {
        assert_eq!(get_byteslicetolerant_from_byterange("Hello", 1usize..3usize), ['e' as u8, 'l' as u8]);
        assert_eq!(get_byteslicetolerant_from_byterange("Hello", 2usize..2usize), []);
        assert_eq!(get_byteslicetolerant_from_byterange("Hello", 3usize..1usize), []);
        assert_eq!(get_byteslicetolerant_from_byterange("Hello", 10usize..12usize), []);
        assert_eq!(get_byteslicetolerant_from_byterange("Hello", 2usize..12usize), ['l' as u8, 'l' as u8,'o' as u8]);
    }


    #[test]
    pub fn test_byteslice_from_byterangeinclusive() {
        assert_eq!(get_byteslice_from_byterangeinclusive("Hello", 1usize..=3usize), ['e' as u8, 'l' as u8, 'l' as u8]);
        assert_eq!(get_byteslice_from_byterangeinclusive("Hello", 2usize..=2usize), ['l' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_byterangefrom() {
        assert_eq!(get_byteslice_from_byterangefrom("Hello", 2usize..), ['l' as u8, 'l' as u8, 'o' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_byterangeto() {
        assert_eq!(get_byteslice_from_byterangeto("Hello", ..2usize), ['H' as u8, 'e' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_byterangetoinclusive() {
        assert_eq!(get_byteslice_from_byterangetoinclusive("Hello", ..=2usize), ['H' as u8, 'e' as u8, 'l' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_bytefullrange() {
        assert_eq!(get_byteslice_from_bytefullrange("Hello", ..), ['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_startbytecount() {
        assert_eq!(get_byteslice_from_startbytecount("Hello", 3usize), ['H' as u8, 'e' as u8, 'l' as u8]);
    }

    #[test]
    pub fn test_byteslice_from_endbytecount() {
        assert_eq!(get_byteslice_from_endbytecount("Hello", 3usize), ['l' as u8, 'l' as u8, 'o' as u8]);
    }
    

    // ------------------------
    // get_bytevector, copying bytes

    // Basic version, no range
    #[test]
    pub fn test_bytevector() {
        assert_eq!(get_bytevector("ABC"), vec!['A' as u8, 'B' as u8, 'C' as u8]);
    }
    
    // Returning a Vec<u8> is Ok, but it duplicates characters
    #[test]
    pub fn test_bytevector_from_byterange() {
        assert_eq!(get_bytevector_from_byterange("Hello", 2usize..4usize), vec!['l' as u8, 'l' as u8]);
    }
    
    #[test]
    pub fn test_bytevector_from_byterangeinclusive() {
        assert_eq!(get_bytevector_from_byterangeinclusive("Hello", 0usize..=3usize), vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8])
    }
    
    // and many range variants
    
    // ----------------------------------
    // get byteiterator
    
    /*
    // Basic version, no range
    pub fn get_byteiterator<'a>(s: &'a str) -> impl Iterator<Item = u8> + 'a {
        s.bytes()
    }
    
    // Returning an iterator on bytes
    pub fn get_byteiterator_from_byterange<'a>(s: &'a str, byterange: &Range<usize>) -> impl Iterator<Item = u8> + 'a {
        s.as_bytes()[byterange.clone()].iter().copied()
    }
    
    pub fn get_byteiterator_from_byterangeinclusive<'a>(s: &'a str, byterange: &RangeInclusive<usize>) -> impl Iterator<Item = u8> + 'a {
        s.as_bytes()[byterange.clone()].iter().copied()
    }
     */
    // and many variants
  
}
