// unit tests for vstring
// Learning rust
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod byteindex_tests {
    #[test]
    fn get_byte_from_byteindex_tests() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(l19_vstring::get_byte_from_byteindex(s, 0), 65);
    }    
}

