use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct StableOrderedSet<T>
where
    T: Eq + Hash + Clone,
{
    inner_map: HashMap<T, usize>,
    values: Vec<T>,
}

impl<T: Eq + Hash + Clone> StableOrderedSet<T> {
    pub fn new() -> Self {
        let inner_map = HashMap::new();
        let values_vec = Vec::new();
        StableOrderedSet {
            inner_map,
            values: values_vec,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner_map.contains_key(value)
    }

    pub fn insert(&mut self, item: T) {
        if !self.inner_map.contains_key(&item) {
            let id = self.values.len();
            let item_copy = item.clone();
            self.values.push(item);
            self.inner_map.insert(item_copy, id);
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        if index < self.values.len() {
            Some(&self.values[index])
        } else {
            None
        }
    }
    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.values.len() {
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    pub fn get_index_of(&self, value: &T) -> Option<usize> {
        self.inner_map.get(value).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.values.into_iter()
    }

    pub fn difference<'a>(&'a self, other: &'a StableOrderedSet<T>) -> StableOrderedSet<T> {
        let mut result = StableOrderedSet::new();

        for item in self.clone() {
            if !other.contains(&item) {
                result.insert(item);
            }
        }

        result
    }

    pub fn union<'a>(&'a self, other: &'a StableOrderedSet<T>) -> StableOrderedSet<T> {
        let mut result = StableOrderedSet::new();

        for item in self {
            result.insert(item);
        }
        for item in other.clone() {
            result.insert(item);
        }

        result
    }
}

impl<T: Eq + Hash + Clone> Index<usize> for StableOrderedSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).unwrap()
    }
}

impl<T: Eq + Hash + Clone> IndexMut<usize> for StableOrderedSet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_index_mut(index).unwrap()
    }
}

impl<T: Eq + Hash + Clone> IntoIterator for StableOrderedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
impl<T: Eq + Hash + Clone> IntoIterator for &StableOrderedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.clone().into_iter()
    }
}

impl<T: Eq + Hash + Clone> FromIterator<T> for StableOrderedSet<T> {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut collection = StableOrderedSet::new();
        for item in iter {
            collection.insert(item);
        }

        collection
    }
}
impl<T, const N: usize> From<[T; N]> for StableOrderedSet<T>
where
    T: Eq + Hash + Clone,
{
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}
