use std::collections::{HashMap, HashSet};
use std::collections::hash_map::{Keys,Values};
use std::hash::Hash;
use std::cmp::max;

use super::ast::Scope;

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
#[unstable(feature = "forktable")]
pub struct ForkTable<'a, K:'a +  Eq + Hash,V: 'a>  {
    table: HashMap<K, V>,
    whiteouts: HashSet<K>,
    parent: Option<&'a ForkTable<'a, K,V>>,
    level: usize
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
    /// # Return Value
    ///
    ///  + `Some(&V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.get(&1isize), Some(&"One"));
    /// assert_eq!(table.get(&2isize), None);
    /// ```
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1isize, "One");
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.get(&1isize), Some(&"One"));
    /// ```
    #[stable(feature = "forktable", since = "0.0.3")]
    pub fn get<'b>(&'b self, key: &K) -> Option<&'b V> {
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
    /// # Return Value
    ///
    ///  + `Some(&mut V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get_mut(&1isize), None);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.get_mut(&1isize), Some(&mut "One"));
    /// assert_eq!(table.get_mut(&2isize), None);
    /// ```
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1isize, "One");
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.get_mut(&1isize), None);
    /// ```
   #[unstable(feature = "forktable")]
   pub fn get_mut<'b>(&'b mut self, key: &K) -> Option<&'b mut V> {
        self.table.get_mut(key)
    }


    /// Removes a key from the map, returning the value at the key if
    /// the key was previously in the map.
    ///
    /// If the removed value exists in a lower level of the table,
    /// it will be whited out at this level. This means that the entry
    /// will be 'removed' at this level and this table will not provide
    /// access to it, but the mapping will still exist in the level where
    /// it was defined. Note that the key will not be returned if it is
    /// defined in a lower level of the table.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// `Hash` and `Eq` on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Arguments
    ///
    ///  + `key`  - the key to remove
    ///
    /// # Return Value
    ///
    ///  + `Some(V)` if an entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// table.insert(1isize, "One");
    ///
    /// assert_eq!(table.remove(&1isize), Some("One"));
    /// assert_eq!(table.contains_key(&1isize), false);
    /// ```
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// level_1.insert(1isize, "One");
    /// assert_eq!(level_1.contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1isize), true);
    /// assert_eq!(level_2.remove(&1isize), None);
    /// assert_eq!(level_2.chain_contains_key(&1isize), false);
    /// ```
    ///
    #[unstable(feature = "forktable")]
    pub fn remove(&mut self, key: &K) -> Option<V> where K: Clone {
            if self.table.contains_key(key) {
                self.table.remove(key)
            } else if self.chain_contains_key(key) {
                self.whiteouts.insert(key.clone()); // TODO: could just white out specific hashes?
                None
            } else {
                None
            }
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
    ///  + `k`  - the key to add
    ///  + `v`  - the value to associate with that key
    ///
    /// # Return Value
    ///
    ///  + `Some(V)` if a previous entry for the given key exists in the
    ///     table, or `None` if there is no entry for that key.
    ///
    /// # Examples
    ///
    /// Simply inserting an entry:
    ///
    /// ```
    /// # #![feature(forktable,scheme)]
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
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// assert_eq!(table.insert(1isize, "two"), None);
    /// assert_eq!(table.get(&1isize), Some(&"two"));
    ///
    /// assert_eq!(table.insert(2isize, "Two"), Some("two"));
    /// assert_eq!(table.get(&2isize), Some(&"Two"));
    /// ```
    #[stable(feature = "forktable", since = "0.0.3")]
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
    /// # Return Value
    ///
    ///  + `true` if the given key is defined in this level of the
    ///    table, `false` if it does not.
    ///
    /// # Examples
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.contains_key(&1isize), true);
    /// ```
    /// ```ignore
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.contains_key(&1isize), false);
    /// level_1.insert(1isize, "One");
    /// assert_eq!(level_1.contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.contains_key(&1isize), false);
    /// ```
    #[stable(feature = "forktable", since = "0.0.3")]
    pub fn contains_key(&self, key: &K) -> bool  {
        !self.whiteouts.contains(key)  &&
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
    /// # Return Value
    ///
    ///  + `true` if the given key is defined in the table,
    ///    `false` if it does not.
    ///
    /// # Examples
    /// ```
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.chain_contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.chain_contains_key(&1isize), true);
    /// ```
    /// ```ignore
    /// # #![feature(forktable,scheme)]
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(level_1.chain_contains_key(&1isize), false);
    /// level_1.insert(1isize, "One");
    /// assert_eq!(level_1.chain_contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,&str> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1isize), true);
    /// ```
    #[stable(feature = "forktable", since = "0.0.3")]
    pub fn chain_contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key) ||
        (!self.whiteouts.contains(key) &&
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
    ///
    /// Note that the new `ForkTable<K,V>` has a lifetime
    /// bound ensuring that it will live at least as long as the
    /// parent `ForkTable`.
    ///
    /// TODO: should whiteouts be carried over? look into this.
    #[unstable(feature = "forktable")]
    pub fn fork(&'a self) -> ForkTable<'a, K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: Some(self),
            level: self.level + 1
        }
    }

    /// Constructs a new `ForkTable<K,V>`
    #[stable(feature = "forktable",since="0.0.3")]
    pub fn new() -> ForkTable<'a, K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: None,
            level: 0
        }
    }

    /// Wrapper for the backing map's `values()` function.
    ///
    /// Provides an iterator visiting all values in arbitrary
    /// order. Iterator element type is &'b V.
    #[unstable(feature="forktable")]
    pub fn values<'b>(&'b self) -> Values<'b, K, V> {
        self.table.values()
    }

    /// Wrapper for the backing map's `keys()` function.
    ///
    /// Provides an iterator visiting all keys in arbitrary
    /// order. Iterator element type is &'b K.
    #[unstable(feature="forktable")]
    pub fn keys<'b>(&'b self) -> Keys<'b, K, V>{
        self.table.keys()
    }
}

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
#[stable(feature = "compile",since = "0.1.0")]
impl<'a> Scope<&'a str> for ForkTable<'a, &'a str, (usize,usize)> {
    /// Bind a name to a scope.
    ///
    /// Returns the indices for that name in the SVM environment.
    #[stable(feature = "compile",since = "0.1.0")]
    fn bind(&mut self,name: &'a str, lvl: usize) -> (usize,usize) {
        let idx = self.values().fold(0, |a,i| max(a,i.1)) + 1;
        self.insert(name, (lvl,idx));
        (self.level, idx)
    }
    /// Look up a name against a scope.
    ///
    /// Returns the indices for that name in the SVM environment,
    /// or None if that name is unbound.
    #[stable(feature = "compile",since = "0.1.0")]
    fn lookup(&self, name: &&'a str)             -> Option<(usize,usize)> {
        match self.get(name) {
            Some(&(lvl,idx)) => Some((lvl.clone(), idx.clone())),
            None             => None
        }
    }

}
