use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

#[derive(Debug, Default, Clone)]
pub struct TextureBuilder {
    colors: Vec<Color>,
}

impl TextureBuilder {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    pub fn image_from_colors(colors: Vec<Color>) -> Image {
        let builder = Self::new(colors);
        builder.build_texture_image()
    }

    pub fn build_texture_image(&self) -> Image {
        let texture_data = self.colors_to_flat_array();

        if self.colors.len() == 0 {
            warn!("WARNING: Color length is 0!");
        }

        Image::new(
            Extent3d {
                width: self.colors.len() as u32,
                ..default()
            },
            TextureDimension::D2,
            texture_data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        )
    }

    fn colors_to_flat_array(&self) -> Vec<u8> {
        self.colors
            .iter()
            .flat_map(|color| color.to_srgba().to_u8_array())
            .collect()
    }
}
