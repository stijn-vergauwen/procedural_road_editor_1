pub mod buttons;
pub mod content_size;
pub mod content_wrap;
pub mod flexbox;
pub mod inputs;
pub mod section;
pub mod text;

use bevy::prelude::*;
use buttons::ButtonsPlugin;
use inputs::InputComponentsPlugin;

pub struct UiComponentsPlugin;

impl Plugin for UiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonsPlugin, InputComponentsPlugin));
    }
}

pub trait UiComponentBuilder: Default {
    /// Spawn this component with the current config.
    ///
    /// - the `components` param is for any extra components you want to add to the spawned entity, leave this empty `()` if you don't need it.
    fn spawn(&self, builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        builder.spawn((self.build(), components)).id()
    }

    /// Shorthand for spawning this component with default config.
    #[expect(unused)]
    fn spawn_default(builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        Self::default().spawn(builder, components)
    }

    fn build(&self) -> impl Bundle;
}

pub trait UiComponentWithChildrenBuilder: Default {
    /// Spawn this component with the current config.
    ///
    /// - the `components` param is for any extra components you want to add to the spawned entity, leave this empty `()` if you don't need it.
    fn spawn(
        &self,
        builder: &mut ChildBuilder,
        components: impl Bundle,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        builder
            .spawn((self.build(), components))
            .with_children(children)
            .id()
    }

    /// Shorthand for spawning this component with default config.
    fn spawn_default(
        builder: &mut ChildBuilder,
        components: impl Bundle,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        Self::default().spawn(builder, components, children)
    }

    fn build(&self) -> impl Bundle;
}
