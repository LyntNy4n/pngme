use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.first_byte,
            self.second_byte,
            self.third_byte,
            self.fourth_byte,
        ]
    }
    pub fn is_critical(&self) -> bool {
        self.first_byte & 0b0010_0000 == 0
    }
    pub fn is_public(&self) -> bool {
        self.second_byte & 0b0010_0000 == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.third_byte & 0b0010_0000 == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.fourth_byte & 0b0010_0000 != 0
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

fn is_all_alphabetic(bytes: [u8; 4]) -> bool {
    bytes.iter().all(|byte| byte.is_ascii_alphabetic())
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; 4]) -> std::result::Result<Self, Self::Error> {
        if !is_all_alphabetic(bytes) {
            return Err("ChunkType must be alphabetic");
        }
        let chunk_type = ChunkType {
            first_byte: bytes[0],
            second_byte: bytes[1],
            third_byte: bytes[2],
            fourth_byte: bytes[3],
        };
        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("ChunkType string must be 4 characters long");
        }
        let mut bytes = [0; 4];
        for (i, byte) in s.bytes().enumerate() {
            bytes[i] = byte;
        }
        Self::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.bytes();
        for byte in bytes.iter() {
            write!(f, "{}", *byte as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
