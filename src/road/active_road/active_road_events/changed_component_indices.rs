use crate::{
    ui::list::list_events::list_reorder::ReorderIndices, utility::changed_value::ChangedValue,
};

#[derive(Clone, PartialEq, Debug)]
pub struct ChangedComponentIndices(pub Vec<ChangedValue<Option<usize>>>);

impl ChangedComponentIndices {
    pub fn new(values: Vec<ChangedValue<Option<usize>>>) -> Self {
        for value in values.iter() {
            assert!(
                value.previous_value.is_some() || value.new_value.is_some(),
                "Tried to initialize struct with a ChangedValue where previous and new values are both None"
            );
        }

        Self(values)
    }

    pub fn from_reorder(reorder: ReorderIndices) -> Self {
        Self::new(vec![ChangedValue::new(
            Some(reorder.previous_index),
            Some(reorder.new_index),
        )])
    }

    /// Returns an iterator over all the newly added indices.
    pub fn iter_added(&self) -> impl Iterator<Item = &ChangedValue<Option<usize>>> {
        self.0.iter().filter(|item| item.previous_value.is_none())
    }

    /// Returns an iterator over all the deleted indices.
    pub fn iter_deleted(&self) -> impl Iterator<Item = &ChangedValue<Option<usize>>> {
        self.0.iter().filter(|item| item.new_value.is_none())
    }

    /// Returns an iterator over all the indices that already existed and aren't deleted.
    pub fn iter_existing(&self) -> impl Iterator<Item = ChangedValue<usize>> + '_ {
        self.0
            .iter()
            .filter_map(|item| Some(ChangedValue::new(item.previous_value?, item.new_value?)))
    }

    /// Returns the new value for the given index, or None if the index is deleted.
    ///
    /// If the given index isn't present in the data, it will assume the index stayed the same.
    pub fn map_index(&self, index: usize) -> Option<usize> {
        for item in self.0.iter() {
            if item.previous_value == Some(index) {
                return item.new_value;
            }
        }

        Some(index)
    }
}
