use crate::*;

pub fn hash_map_xx_hash3_64<K: Eq + Hash, V>() -> HashMapXxHash3_64<K, V> {
    HashMap::with_hasher(BuildHasherDefault::default())
}
