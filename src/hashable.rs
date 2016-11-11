/// By definition, data in the Merkle Tree must implement Hashable
pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Hashable implementation for standard String type
impl Hashable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}

// Implementation for standard u8 type
impl Hashable for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

// Implementation for standard &str type
impl Hashable for &'static str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
