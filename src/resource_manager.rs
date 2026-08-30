use macroquad::{
    text::{Font, load_ttf_font_from_bytes},
    texture::Texture2D,
};

use crate::level::{AboveTile, FloorTile};

const EMPTY_IMG: &[u8] = include_bytes!("../resources/images/empty.png");
const BOX_EXIT_IMG: &[u8] = include_bytes!("../resources/images/box_exit.png");
const WALL_IMG: &[u8] = include_bytes!("../resources/images/wall.png");
const PALYER_IMG: &[u8] = include_bytes!("../resources/images/player.png");
const BOX_IMG: &[u8] = include_bytes!("../resources/images/box.png");

const LEVEL_BUTTON_IMG: &[u8] = include_bytes!("../resources/images/level_button.png");
const LEVEL_BUTTON_SELECTED_IMG: &[u8] =
    include_bytes!("../resources/images/level_button_selected.png");

const FONT_BYTES: &[u8] = include_bytes!("../resources/font.ttf");

#[derive(Debug)]
pub struct ResourceManager {
    pub font: Font,
    pub level_button: Texture2D,
    pub level_button_selected: Texture2D,
    empty_tile: Texture2D,
    box_exit_tile: Texture2D,
    wall_tile: Texture2D,
    player_tile: Texture2D,
    box_tile: Texture2D,
}
impl ResourceManager {
    pub fn new() -> Self {
        Self {
            font: Self::load_font(),
            level_button: Self::load(LEVEL_BUTTON_IMG),
            level_button_selected: Self::load(LEVEL_BUTTON_SELECTED_IMG),
            empty_tile: Self::load(EMPTY_IMG),
            box_exit_tile: Self::load(BOX_EXIT_IMG),
            wall_tile: Self::load(WALL_IMG),
            player_tile: Self::load(PALYER_IMG),
            box_tile: Self::load(BOX_IMG),
        }
    }

    pub fn get_texture_for_floor(&self, floor: FloorTile) -> &Texture2D {
        match floor {
            FloorTile::None => &self.empty_tile,
            FloorTile::BoxExit => &self.box_exit_tile,
        }
    }

    pub fn get_texture_for_tile(&self, tile: AboveTile) -> &Texture2D {
        match tile {
            AboveTile::None => panic!("Received 'None' tile"),
            AboveTile::Player => &self.player_tile,
            AboveTile::Box => &self.box_tile,
            AboveTile::Wall => &self.wall_tile,
        }
    }

    fn load_font() -> Font {
        load_ttf_font_from_bytes(FONT_BYTES).expect("Error loading font")
    }

    fn load(image_bytes: &[u8]) -> Texture2D {
        let texture = Texture2D::from_file_with_format(image_bytes, None);
        texture.set_filter(macroquad::texture::FilterMode::Nearest);
        texture
    }
}
