use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
pub trait HashOne: Hash
{
    fn hash_one(self: &Self) -> u64
    {
        let mut hash = DefaultHasher::new();
        self.hash(&mut hash);
        hash.finish()
    }
}
impl HashOne for [u8] {}

#[cfg(test)]
mod tests
{
    use super::HashOne;

    #[test]
    fn hash_one_eq()
    {
        let arr0: [u8; 257] = [0; 257];
        let arr1: [u8; 257] = [0; 257];
        assert_eq!(arr0.hash_one(), arr1.hash_one());
    }

    #[test]
    fn hash_one_ne()
    {
        let arr0: [u8; 254] = [0; 254];
        let arr1: [u8; 256] = [0; 256];
        assert_ne!(arr0.hash_one(), arr1.hash_one());
    }
}
