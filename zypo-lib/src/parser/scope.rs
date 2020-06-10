//! Contains the primary scoping structure and various trait implementations.
//! Please see [Scope] for more infomation.
use std::collections::HashMap;
use std::hash::Hash;

/// Structure for storing an AST node as a hashmap node inside of a vector.
///
/// To use this structure, a node type has to implament [Hash] and [Eq] (similar
/// to [PartialEq]).
pub struct Scope<K: Hash + Eq, V>(Vec<HashMap<K, V>>);

impl<K: Hash + Eq, V> Scope<K, V> {
    /// Creates a new, empty scope.
    #[inline]
    pub fn new() -> Self {
        Self(vec![HashMap::new()])
    }

    /// Creates a new scope at top of stack, acting as a new scope.
    #[inline]
    pub fn enter_scope(&mut self) {
        self.0.push(HashMap::<K, V>::new());
    }

    /// Leaves the scope by popping the top value of the [HashMap] vector stack.
    #[inline]
    pub fn leave_scope(&mut self) {
        self.0.pop();
    }

    /// Scans bottom-up from the scope stack to find the specified `item`
    /// argument. This method recurses down the branches of the stack until it
    /// finds the item (linear search).
    #[inline]
    pub fn get(&self, item: &K) -> Option<&V> {
        for stack_item in self.0.iter().rev() {
            let cur_item = stack_item.get(item);
            if cur_item.is_some() {
                return cur_item;
            }
        }

        None
    }

    /// Inserts a new item into the stack.
    ///
    /// NOTE: This will panic if a fatal error occurs if inserting into a broken
    /// stack.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.last_mut().unwrap().insert(key, value)
    }

    /// Gets the depth of the entire [Scope] data structure.
    pub fn depth(&self) -> usize {
        self.0.len()
    }
}
