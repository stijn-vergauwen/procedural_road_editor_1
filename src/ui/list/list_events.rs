pub mod list_item_deletion;
pub mod list_reorder;
pub mod new_list_item;

use bevy::prelude::*;
use list_item_deletion::ListItemDeletionPlugin;
use list_reorder::ListReorderPlugin;
use new_list_item::NewListItemPlugin;

pub struct ListEventsPlugin;

impl Plugin for ListEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NewListItemPlugin, ListReorderPlugin, ListItemDeletionPlugin));
    }
}
