use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::borrow::Borrow;

#[derive(Debug,Clone)]
pub struct ForkTable<K: Eq + Hash,V>  {
    table: HashMap<K, V>,
    whiteouts: HashSet<K>,
    parent: Option<Box<ForkTable<K,V>>>
}

impl<K,V> ForkTable<K, V> where K: Eq + Hash {

    /// Returns a reference to the value corresponding to the key.
    ///
    /// If the key is defined in this level of the table, or in any
    /// of its' parents, a reference to the associated value will be
    /// returned.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>, Q: Hash + Eq {
            unimplemented!()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// If the key is defined in this level of the table, or in any
    /// of its' parents, a reference to the associated value will be
    /// returned.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
        where K: Borrow<Q>, Q: Hash + Eq {
            unimplemented!()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.whiteouts.contains(&k) { self.whiteouts.remove(&k); };
        self.table.insert(k, v)
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    pub fn contains_key(&self, key: &K) -> bool  {
        self.table.contains_key(key)
    }

    /// Returns true if the key is defined in this level of the table, or
    /// in any of its' parents and is not whited out.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    pub fn chain_contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key) ||
        (self.whiteouts.contains(key) &&
            match self.parent {
                Some(box ref p) => p.chain_contains_key(key),
                None    => false
            })
    }

    pub fn fork(self) -> ForkTable<K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: Some(box self)
        }
    }
}