use crate::{
    road::road_component::road_component_position::RoadComponentPosition,
    ui::list::list_events::list_reorder::ReorderIndices, utility::changed_value::ChangedValue,
};

#[derive(Clone, PartialEq, Debug, Default)]
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
        Self::new(vec![
            ChangedValue::new(Some(reorder.previous_index), Some(reorder.new_index)),
            ChangedValue::new(Some(reorder.new_index), Some(reorder.previous_index)),
        ])
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

    /// Returns a vec with the delta position of each road component.
    ///
    /// The components are in the same order as the given `previous_component_positions` vec.
    ///
    /// Newly added components aren't included, and deleted components are None values.
    pub fn calculate_delta_component_positions(
        &self,
        previous_component_positions: &[RoadComponentPosition],
        new_component_positions: &[RoadComponentPosition],
    ) -> Vec<Option<RoadComponentPosition>> {
        previous_component_positions
            .iter()
            .enumerate()
            .map(|(index, previous_component_position)| {
                self.calculate_delta_component_position(
                    index,
                    previous_component_position,
                    new_component_positions,
                )
            })
            .collect()
    }

    pub fn calculate_delta_component_position(
        &self,
        index: usize,
        previous_component_position: &RoadComponentPosition,
        new_component_positions: &[RoadComponentPosition],
    ) -> Option<RoadComponentPosition> {
        let new_component_position = new_component_positions.get(self.map_index(index)?)?;

        let delta_position = *new_component_position - *previous_component_position;
        Some(delta_position)
    }
}
