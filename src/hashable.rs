pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;
}

impl Hashable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}

impl Hashable for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Hashable for &'static str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
