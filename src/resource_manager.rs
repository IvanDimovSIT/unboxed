use std::collections::HashMap;

use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound},
    text::{Font, load_ttf_font_from_bytes},
    texture::Texture2D,
    window::next_frame,
};

use crate::{
    graphics::shader::Shader,
    input::MovementInput,
    level::{AboveTile, FloorTile},
    ui::message::draw_loading_screen,
};

const EMPTY_IMG: &[u8] = include_bytes!("../resources/images/empty.png");
const BOX_EXIT_IMG: &[u8] = include_bytes!("../resources/images/box_exit.png");
const PLAYER_EXIT_IMG: &[u8] = include_bytes!("../resources/images/player_exit.png");
const WALL_IMG: &[u8] = include_bytes!("../resources/images/wall.png");
const PALYER_IMG: &[u8] = include_bytes!("../resources/images/player.png");
const BOX_IMG: &[u8] = include_bytes!("../resources/images/box.png");
const PULL_BOX_IMG: &[u8] = include_bytes!("../resources/images/pull_box.png");
const BOMB_IMG: &[u8] = include_bytes!("../resources/images/bomb.png");
const EXPLOSION_IMG: &[u8] = include_bytes!("../resources/images/explosion.png");

const LEVEL_BUTTON_IMG: &[u8] = include_bytes!("../resources/images/level_button.png");
const LEVEL_BUTTON_SELECTED_IMG: &[u8] =
    include_bytes!("../resources/images/level_button_selected.png");
const BACK_ARROW_IMG: &[u8] = include_bytes!("../resources/images/back_arrow.png");
const RESET_X_IMG: &[u8] = include_bytes!("../resources/images/reset_x.png");
const UNDO_ARROW_IMG: &[u8] = include_bytes!("../resources/images/undo_arrow.png");
const BACKGROUND_IMG: &[u8] = include_bytes!("../resources/images/background.png");
const CHECKMARK_IMG: &[u8] = include_bytes!("../resources/images/checkmark.png");
const SOUND_ON_IMG: &[u8] = include_bytes!("../resources/images/sound_on.png");
const SOUND_OFF_IMG: &[u8] = include_bytes!("../resources/images/sound_off.png");
const FULLSCREEN_ON_ARROWS_IMG: &[u8] =
    include_bytes!("../resources/images/fullscreen_on_arrows.png");
const FULLSCREEN_OFF_ARROWS_IMG: &[u8] =
    include_bytes!("../resources/images/fullscreen_off_arrows.png");
const ROTATE_DEVICE_IMG: &[u8] = include_bytes!("../resources/images/rotate_device.png");
const UP_ARROW_IMG: &[u8] = include_bytes!("../resources/images/up_arrow.png");
const DOWN_ARROW_IMG: &[u8] = include_bytes!("../resources/images/down_arrow.png");
const LEFT_ARROW_IMG: &[u8] = include_bytes!("../resources/images/left_arrow.png");
const RIGHT_ARROW_IMG: &[u8] = include_bytes!("../resources/images/right_arrow.png");

const FONT_BYTES: &[u8] = include_bytes!("../resources/font.ttf");

const BUTTON_SOUND: &[u8] = include_bytes!("../resources/sounds/button.wav");
const PUSH_BOX_SOUND: &[u8] = include_bytes!("../resources/sounds/push_box.wav");
const WIN_SOUND: &[u8] = include_bytes!("../resources/sounds/win.wav");
const MOVE_SOUND: &[u8] = include_bytes!("../resources/sounds/move.wav");
const EXPLOSION_SOUND: &[u8] = include_bytes!("../resources/sounds/explosion.wav");

const SOUNDS_ARRAY: [(SoundId, &[u8]); 5] = [
    (SoundId::Button, BUTTON_SOUND),
    (SoundId::PushBox, PUSH_BOX_SOUND),
    (SoundId::Win, WIN_SOUND),
    (SoundId::Move, MOVE_SOUND),
    (SoundId::Explosion, EXPLOSION_SOUND),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundId {
    Button,
    PushBox,
    Win,
    Move,
    Explosion,
}

#[derive(Debug)]
pub struct ResourceManager {
    pub font: Font,
    pub shader: Shader,
    pub level_button: Texture2D,
    pub level_button_selected: Texture2D,
    pub back_arrow: Texture2D,
    pub undo_arrow: Texture2D,
    pub reset_x: Texture2D,
    pub background: Texture2D,
    pub checkmark: Texture2D,
    pub sound_on: Texture2D,
    pub sound_off: Texture2D,
    pub fullscreen_on_arrows: Texture2D,
    pub fullscreen_off_arrows: Texture2D,
    pub explosion: Texture2D,
    pub rotate_device_prompt: Texture2D,
    sounds: HashMap<SoundId, Sound>,
    empty_tile: Texture2D,
    box_exit_tile: Texture2D,
    player_exit_tile: Texture2D,
    wall_tile: Texture2D,
    player_tile: Texture2D,
    box_tile: Texture2D,
    pull_box_tile: Texture2D,
    bomb_tile: Texture2D,
    up_arrow: Texture2D,
    down_arrow: Texture2D,
    left_arrow: Texture2D,
    right_arrow: Texture2D,
}
impl ResourceManager {
    pub async fn new() -> Self {
        let font = Self::load_font();
        draw_loading_screen(&font);
        next_frame().await;

        Self {
            font,
            shader: Shader::new(),
            level_button: Self::load(LEVEL_BUTTON_IMG),
            level_button_selected: Self::load(LEVEL_BUTTON_SELECTED_IMG),
            empty_tile: Self::load(EMPTY_IMG),
            box_exit_tile: Self::load(BOX_EXIT_IMG),
            player_exit_tile: Self::load(PLAYER_EXIT_IMG),
            wall_tile: Self::load(WALL_IMG),
            player_tile: Self::load(PALYER_IMG),
            box_tile: Self::load(BOX_IMG),
            pull_box_tile: Self::load(PULL_BOX_IMG),
            bomb_tile: Self::load(BOMB_IMG),
            background: Self::load(BACKGROUND_IMG),
            explosion: Self::load(EXPLOSION_IMG),
            sounds: Self::load_sounds().await,
            back_arrow: Self::load(BACK_ARROW_IMG),
            checkmark: Self::load(CHECKMARK_IMG),
            reset_x: Self::load(RESET_X_IMG),
            undo_arrow: Self::load(UNDO_ARROW_IMG),
            sound_on: Self::load(SOUND_ON_IMG),
            sound_off: Self::load(SOUND_OFF_IMG),
            fullscreen_on_arrows: Self::load(FULLSCREEN_ON_ARROWS_IMG),
            fullscreen_off_arrows: Self::load(FULLSCREEN_OFF_ARROWS_IMG),
            rotate_device_prompt: Self::load(ROTATE_DEVICE_IMG),
            up_arrow: Self::load(UP_ARROW_IMG),
            down_arrow: Self::load(DOWN_ARROW_IMG),
            left_arrow: Self::load(LEFT_ARROW_IMG),
            right_arrow: Self::load(RIGHT_ARROW_IMG),
        }
    }

    pub fn play_sound(&self, sound_id: SoundId) {
        let sound = &self.sounds[&sound_id];
        play_sound(sound, PlaySoundParams::default());
    }

    pub fn get_arrow_button_texture_for_direction(
        &self,
        movemnt_input: MovementInput,
    ) -> &Texture2D {
        match movemnt_input {
            MovementInput::Left => &self.left_arrow,
            MovementInput::Right => &self.right_arrow,
            MovementInput::Down => &self.down_arrow,
            MovementInput::Up => &self.up_arrow,
            _ => panic!("Invalid direction: {:?}", movemnt_input),
        }
    }

    pub fn get_texture_for_floor(&self, floor: FloorTile) -> &Texture2D {
        match floor {
            FloorTile::None => &self.empty_tile,
            FloorTile::BoxExit => &self.box_exit_tile,
            FloorTile::PlayerExit => &self.player_exit_tile,
        }
    }

    pub fn get_texture_for_tile(&self, tile: AboveTile) -> &Texture2D {
        match tile {
            AboveTile::None => panic!("Received 'None' tile"),
            AboveTile::Player => &self.player_tile,
            AboveTile::Box => &self.box_tile,
            AboveTile::Wall => &self.wall_tile,
            AboveTile::PullBox => &self.pull_box_tile,
            AboveTile::Bomb => &self.bomb_tile,
        }
    }

    async fn load_sounds() -> HashMap<SoundId, Sound> {
        let mut hash_map = HashMap::with_capacity(SOUNDS_ARRAY.len());
        for (id, bytes) in SOUNDS_ARRAY {
            let sound = load_sound_from_bytes(bytes)
                .await
                .expect("Error loading sounds");
            hash_map.insert(id, sound);
        }

        hash_map
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
