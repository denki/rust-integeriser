use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map;
use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::vec::Vec;

pub trait Integeriser {
    type Item;

    /// Returns a unique integer for the given value `a: Self::Item`.  The returned integer will always be the same for equal (w.r.t. `Eq`) values `a` and different for different (w.r.t. `Eq`) values `a`.
    /// The integers are assigned consecutively starting from `0`.
    fn integerise(&mut self, a: Self::Item) -> usize;

    /// Lookup the value that corresponds to the integer `k: usize`.
    fn find_value(&self, k: usize) -> Option<&Self::Item>;

    /// Lookup the integer that corresponds to the value `a: Self::Item`.
    fn find_key(&self, a: &Self::Item) -> Option<usize>;

    /// Number of distinct values `a: Self::Item` that are stored in the iterator.
    fn size(&self) -> usize;
}

/// Structure that maps to every element of type `A` an integer of type `usize`, given that `A: Eq + Hash`.  Mapping goes both ways.
///
/// # Example
///
/// ```
/// use integeriser::{Integeriser, HashIntegeriser};
///
/// let arr1 = vec!["this", "is", "a", "test", "."];
/// let arr2 = vec!["this", "test", "is", "really", "simple", "."];
///
/// let mut integeriser = HashIntegeriser::new();
///
/// let arr1i: Vec<usize> = arr1.iter().map(|w| integeriser.integerise(w)).collect();
/// let arr2i: Vec<usize> = arr2.iter().map(|w| integeriser.integerise(w)).collect();
///
/// assert_eq!(arr1i[0], arr2i[0]);
/// assert_eq!(arr1i[1], arr2i[2]);
/// assert_eq!(arr1i[3], arr2i[1]);
/// assert_eq!(arr1i[4], arr2i[5]);
///
/// assert_ne!(arr1i[1], arr2i[0]);
/// assert_ne!(arr1i[2], arr2i[1]);
/// assert_ne!(arr1i[3], arr2i[3]);
/// ```
#[derive(Clone, Debug)]
pub struct HashIntegeriser<A: Eq + Hash> {
    map: Vec<A>,
    rmap: HashMap<A, usize>,
}

impl<A: Eq + Hash> HashIntegeriser<A> {
    /// Constructs a new, empty `HashIntegeriser<A>`.
    pub fn new() -> HashIntegeriser<A> {
        HashIntegeriser { map: Vec::new(), rmap: HashMap::new() }
    }

    /// `Vec` containing all the values that have been stored in the iterator.
    pub fn values(&self) -> &Vec<A>{
        &self.map
    }
}

impl<'a, A: Clone + Eq + Hash> Integeriser for HashIntegeriser<A> {
    type Item = A;

    fn integerise(&mut self, a: A) -> usize {
        match self.rmap.entry(a) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let old_size = self.map.len();
                self.map.push(e.key().clone());
                e.insert(old_size);
                old_size
            }
        }
    }

    fn find_value(&self, k: usize) -> Option<&A> {
        self.map.get(k)
    }

    fn find_key(&self, a: &A) -> Option<usize> {
        self.rmap.get(a).cloned()
    }

    fn size(&self) -> usize {
        self.map.len()
    }
}

impl<A: Eq + Hash> PartialEq for HashIntegeriser<A> {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<A: Eq + Hash> Eq for HashIntegeriser<A> {}

impl<A: Eq + Hash + PartialOrd> PartialOrd for HashIntegeriser<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.map.partial_cmp(&other.map)
    }
}

impl<A: Eq + Hash + Ord> Ord for HashIntegeriser<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.map.cmp(&other.map)
    }
}

impl<A: Eq + Hash> Hash for HashIntegeriser<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.hash(state);
    }
}


/// Structure that maps to every element of type `A` an integer of type `usize`, given that `A: Eq + Ord`.  Mapping goes both ways.
///
/// # Example
///
/// ```
/// use integeriser::{Integeriser, BTreeIntegeriser};
///
/// let arr1 = vec!["this", "is", "a", "test", "."];
/// let arr2 = vec!["this", "test", "is", "really", "simple", "."];
///
/// let mut integeriser = BTreeIntegeriser::new();
///
/// let arr1i: Vec<usize> = arr1.iter().map(|w| integeriser.integerise(w)).collect();
/// let arr2i: Vec<usize> = arr2.iter().map(|w| integeriser.integerise(w)).collect();
///
/// assert_eq!(arr1i[0], arr2i[0]);
/// assert_eq!(arr1i[1], arr2i[2]);
/// assert_eq!(arr1i[3], arr2i[1]);
/// assert_eq!(arr1i[4], arr2i[5]);
///
/// assert_ne!(arr1i[1], arr2i[0]);
/// assert_ne!(arr1i[2], arr2i[1]);
/// assert_ne!(arr1i[3], arr2i[3]);
/// ```
#[derive(Clone, Debug)]
pub struct BTreeIntegeriser<A: Ord + Eq> {
    map: Vec<A>,
    rmap: BTreeMap<A, usize>,
}

impl<A: Eq + Ord> BTreeIntegeriser<A> {
    /// Constructs a new, empty `BTreeIntegeriser<A>`.
    pub fn new() -> BTreeIntegeriser<A> {
        BTreeIntegeriser { map: Vec::new(), rmap: BTreeMap::new() }
    }

    /// `Vec` containing all the values that have been stored in the iterator.
    pub fn values(&self) -> &Vec<A>{
        &self.map
    }
}

impl<A: Eq + Ord + Clone> Integeriser for BTreeIntegeriser<A> {
    type Item = A;

    fn integerise(&mut self, a: A) -> usize {
        match self.rmap.entry(a) {
            btree_map::Entry::Occupied(e) => *e.get(),
            btree_map::Entry::Vacant(e) => {
                let old_size = self.map.len();
                self.map.push(e.key().clone());
                e.insert(old_size);
                old_size
            }
        }
    }

    fn find_value(&self, k: usize) -> Option<&A> {
        self.map.get(k)
    }

    fn find_key(&self, a: &A) -> Option<usize> {
        self.rmap.get(a).cloned()
    }

    fn size(&self) -> usize {
        self.map.len()
    }
}

impl<A: Eq + Ord> PartialEq for BTreeIntegeriser<A> {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<A: Eq + Ord> Eq for BTreeIntegeriser<A> {}

impl<A: Eq + Ord> PartialOrd for BTreeIntegeriser<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: Eq + Ord> Ord for BTreeIntegeriser<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.map.cmp(&other.map)
    }
}

impl<A: Eq + Ord + Hash> Hash for BTreeIntegeriser<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.hash(state);
    }
}

