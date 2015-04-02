use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::borrow::Borrow;

/// An associative map data structure for representing scopes.
///
/// A `ForkTable` functions similarly to a standard associative map
/// data structure (such as a `HashMap`), but with the ability to
/// fork children off of each level of the map. If a key exists in any
/// of a child's parents, the child will 'pass through' that key. If a
/// new value is bound to a key in a child level, that child will overwrite
/// the previous entry with the new one, but the previous `key` -> `value`
/// mapping will remain in the level it is defined. This means that the parent
/// level will still provide the previous value for that key.
///
/// This is an implementation of the ForkTable data structure for
/// representing scopes. The ForkTable was initially described by
/// Max Clive. This implemention is based primarily by the Scala
/// reference implementation written by Hawk Weisman for the Decaf
/// compiler, which is available [here](https://github.com/hawkw/decaf/blob/master/src/main/scala/com/meteorcode/common/ForkTable.scala).
#[derive(Debug)]
pub struct ForkTable<'a, K:'a +  Eq + Hash,V: 'a>  {
    table: HashMap<K, V>,
    whiteouts: HashSet<K>,
    parent: Option<&'a mut ForkTable<'a, K,V>>
}

impl<'a, K,V> ForkTable<'a, K, V> where K: Eq + Hash {

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
    /// # Arguments
    ///
    ///  + `key`  - the key to search for
    ///
    pub fn get<'b, Q: ?Sized>(&'b self, key: &Q) -> Option<&'b V>
        where K: Borrow<Q>, Q: Hash + Eq {
        if self.whiteouts.contains(key) {
            None
        } else {
            self.table
                .get(key)
                .or(match self.parent {
                        Some(ref parent)    => parent.get(key),
                        None                => None
                    })
        }
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
    /// # Arguments
    ///
    ///  + `key`  - the key to search for
    ///
   pub fn get_mut<'b, Q: ?Sized>(&'b mut self, key: &Q) -> Option<&'b mut V>
        where K: Borrow<Q>, Q: Hash + Eq {
        if self.whiteouts.contains(key) {
            None
        } else {
            self.table
                .get_mut(key)
                .or(match self.parent {
                        Some(ref mut parent)    => parent.get_mut(key),
                        None                    => None
                    })
        }
    }


    /// Removes a key from the map, returning the value at the key if
    /// the key was previously in the map.
    ///
    /// If the removed value exists in a lower level of the table,
    /// it will be whited out at this level. This means that the entry
    /// will be 'removed' at this level and this table will not provide
    /// access to it, but the mapping will still exist in the level where
    /// it was defined.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where K: Borrow<Q>, Q: Hash + Eq {
            unimplemented!()
    }

    /// Inserts a key-value pair from the map.
    ///
    /// If the key already had a value present in the map, that
    /// value is returned. Otherwise, `None` is returned.
    ///
    /// If the key is currently whited out (i.e. it was defined
    /// in a lower level of the map and was removed) then it will
    /// be un-whited out and added at this level.
    ///
    /// # Arguments
    ///
    ///  + `k`  - the key
    ///  + `v`  - the value
    ///
    /// # Examples
    ///
    /// Simply inserting an entry:
    ///
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.get(&1isize), Some(&"One"));
    /// ```
    ///
    /// Overwriting the value associated with a key:
    ///
    /// ```ignore
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// assert_eq!(table.insert(1isize, "two"), None);
    /// assert_eq!(table.get(&1isize), Some(&"two"));
    ///
    /// assert_eq!(table.insert(2isize, "Two"), Some("two"));
    /// assert_eq!(table.get(&2isize), Some(&"Two"));
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.whiteouts.contains(&k) { self.whiteouts.remove(&k); };
        self.table.insert(k, v)
    }

    /// Returns true if this level contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `k`  - the key to search for
    ///
    /// # Examples
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.contains_key(&1isize), true);
    /// ```
    /// ```ignore
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.contains_key(&1isize), false);
    /// level_1.insert(1isize, "One");
    /// assert_eq!(level_1.contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.contains_key(&1isize), false);
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
    /// # Arguments
    ///
    ///  + `k`  - the key to search for
    ///
    /// # Examples
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.chain_contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.chain_contains_key(&1isize), true);
    /// ```
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.chain_contains_key(&1isize), false);
    /// level_1.insert(1isize, "One");
    /// assert_eq!(level_1.chain_contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1isize), true);
    /// ```
    pub fn chain_contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key) ||
        (self.whiteouts.contains(key) &&
            match self.parent {
                Some(ref p) => p.chain_contains_key(key),
                None        => false
            })
    }

    /// Forks this table, returning a new `ForkTable<K,V>`.
    ///
    /// This level of the table will be set as the child's
    /// parent. The child will be created with an empty backing
    /// `HashMap` and no keys whited out.
    pub fn fork(&'a mut self) -> ForkTable<'a, K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: Some(self)
        }
    }

    /// Constructs a new `ForkTable<K,V>`
    pub fn new() -> ForkTable<'a, K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: None
        }
    }
}