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

    /// Removes a key from the map, returning the value at the key if
    /// the key was previously in the map.
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
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// assert_eq!(table.insert(1isize, "One"), None);
    /// assert_eq!(table.get(&1isize), Some("One"));
    /// ```
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.get(&1isize), None);
    /// assert_eq!(table.insert(1isize, "two"), None);
    /// assert_eq!(table.get(&1isize), Some("two"));
    ///
    /// assert_eq!(table.insert(2isize, "Two"), Some("two"));
    /// ssert_eq!(table.get(&2isize), Some("Two"));
    /// ```
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
    /// # Examples
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut table: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(table.contains_key(&1isize), true);
    /// ```
    /// ```
    /// # use seax_scheme::ForkTable;
    /// let mut level_1: ForkTable<isize,&str> = ForkTable::new();
    /// assert_eq!(table.contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(level_1.contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,String> = level_1.fork();
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
    /// assert_eq!(table.chain_contains_key(&1isize), false);
    /// table.insert(1isize, "One");
    /// assert_eq!(level_1.chain_contains_key(&1isize), true);
    ///
    /// let mut level_2: ForkTable<isize,String> = level_1.fork();
    /// assert_eq!(level_2.chain_contains_key(&1isize), true);
    /// ```
    pub fn chain_contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key) ||
        (self.whiteouts.contains(key) &&
            match self.parent {
                Some(box ref p) => p.chain_contains_key(key),
                None    => false
            })
    }

    /// Forks this table, returning a new `ForkTable<K,V>`.
    ///
    /// This level of the table will be set as the child's
    /// parent. The child will be created with an empty backing
    /// `HashMap` and no keys whited out.
    pub fn fork(self) -> ForkTable<K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: Some(box self)
        }
    }

    /// Constructs a new `ForkTable<K,V>`
    pub fn new() -> ForkTable<K,V> {
        ForkTable {
            table: HashMap::new(),
            whiteouts: HashSet::new(),
            parent: None
        }
    }
}