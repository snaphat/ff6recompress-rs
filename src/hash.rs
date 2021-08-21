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
