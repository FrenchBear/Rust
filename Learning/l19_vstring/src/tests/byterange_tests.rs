// vstring unit tests - Functions based on byte range tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod byterange_tests {
    use glyph2::Glyph2;

    use crate::*;

    // ------------------------
    // get_byte

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

    /*
    // Basic version, no range
    pub fn get_bytevector(s: &str) -> Vec<u8> {
        Vec::from(s.as_bytes())
    }
    
    // Returning a Vec<u8> is Ok, but it duplicates characters
    pub fn get_bytevector_from_byterange(s: &str, byterange: &Range<usize>) -> Vec<u8> {
        Vec::from(&s.as_bytes()[byterange.clone()])
    }
    
    pub fn get_bytevector_from_byterangeinclusive(s: &str, byterange: &RangeInclusive<usize>) -> Vec<u8> {
        Vec::from(&s.as_bytes()[byterange.clone()])
    }
    
    // can add many variants
    
    // ----------------------------------
    // get byteiterator
    
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
    
    // and many variants
    
*/    
}
