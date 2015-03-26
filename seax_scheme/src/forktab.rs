use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct ForkTable<'a,K: 'a,V: 'a>  {
    table: HashMap<K, V>,
    whiteouts: Vec<K>,
    parent: Option<&'a ForkTable<'a,K,V>>
}

impl<'a,K,V> ForkTable<'a, K, V> {

}