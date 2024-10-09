use macroquad::texture::{load_texture, FilterMode, Texture2D};

pub struct Textures {
    pub one: Texture2D,
    pub two: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        let one = load_texture("one.png")
            .await
            .expect("Failed to load one.png texture");
        one.set_filter(FilterMode::Nearest);
        let two = load_texture("two.png")
            .await
            .expect("Failed to load two.png texture");

        two.set_filter(FilterMode::Nearest);

        // This will ensure that all calls to draw_texture() and draw_texture_ex() will use the texture from the atlas instead of each separate texture,
        // which is much more efficient. All textures need to be loaded before this function is called.
        macroquad::texture::build_textures_atlas();

        Textures { one, two }
    }
}
