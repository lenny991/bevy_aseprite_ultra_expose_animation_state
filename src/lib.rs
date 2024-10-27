#[allow(unused)]
use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod loader;
pub(crate) mod slice;

pub struct BevySprityPlugin;
impl Plugin for BevySprityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin);
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
    }
}

pub mod prelude {
    pub use crate::animation::{
        Animation, AnimationDirection, AnimationEvents, AnimationRepeat, AnimationState,
        AsepriteAnimationBundle, AsepriteAnimationUiBundle,
    };
    pub use crate::loader::Aseprite;
    pub use crate::slice::{AsepriteSlice, AsepriteSliceBundle, AsepriteSliceUiBundle};
    pub use crate::BevySprityPlugin;
}

/// tags a bundle as ui node
#[derive(Component, Default)]
pub struct UiTag;

/// tags an entity as not yet loaded;
#[derive(Component, Default)]
pub struct NotLoaded;
