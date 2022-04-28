use serde::{Deserialize, Serialize};

use crate::{
    internal::{
        frontier::{Forget, Full},
        path::Witness,
    },
    AuthPath, Focus, Frontier, GetHash, GetPosition, Hash, Height, Insert,
};

use super::super::complete;

/// The frontier (rightmost) leaf in a frontier of a tree.
///
/// Insertion into a leaf always fails, causing the tree above it to insert a new leaf to contain
/// the inserted item.
#[derive(Clone, Copy, Derivative, Serialize, Deserialize)]
#[derivative(Debug = "transparent")]
pub struct Leaf<Item> {
    item: Item,
}

impl<Item: GetHash> GetHash for Leaf<Item> {
    #[inline]
    fn hash(&self) -> Hash {
        self.item.hash()
    }

    #[inline]
    fn cached_hash(&self) -> Option<Hash> {
        self.item.cached_hash()
    }
}

impl<Item: Height> Height for Leaf<Item> {
    type Height = Item::Height;
}

impl<Item: Frontier + From<Hash>> Frontier for Leaf<Item> {
    type Item = Item;

    #[inline]
    fn new(item: Self::Item) -> Self {
        Self { item }
    }

    #[inline]
    fn update<T>(&mut self, f: impl FnOnce(&mut Self::Item) -> T) -> Option<T> {
        Some(f(&mut self.item))
    }

    #[inline]
    fn focus(&self) -> Option<&Self::Item> {
        Some(&self.item)
    }

    #[inline]
    /// Insertion into a leaf always fails, causing the tree above it to insert a new leaf to
    /// contain the inserted item.
    fn insert_owned(self, item: Self::Item) -> Result<Self, Full<Self>> {
        Err(Full {
            item,
            complete: self.finalize_owned(),
        })
    }

    #[inline]
    fn is_full(&self) -> bool {
        true
    }
}

impl<Item: Focus> Focus for Leaf<Item> {
    type Complete = complete::Leaf<<Item as Focus>::Complete>;

    #[inline]
    fn finalize_owned(self) -> Insert<Self::Complete> {
        self.item.finalize_owned().map(complete::Leaf::new)
    }
}

impl<Item: Witness> Witness for Leaf<Item> {
    type Item = Item::Item;

    #[inline]
    fn witness(&self, index: impl Into<u64>) -> Option<(AuthPath<Self>, Self::Item)> {
        self.item.witness(index)
    }
}

impl<Item: GetPosition> GetPosition for Leaf<Item> {
    #[inline]
    fn position(&self) -> Option<u64> {
        self.item.position()
    }
}

impl<Item: GetHash + Forget> Forget for Leaf<Item> {
    #[inline]
    fn forget(&mut self, index: impl Into<u64>) -> bool {
        self.item.forget(index)
    }
}
