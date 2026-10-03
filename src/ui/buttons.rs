use macroquad::{
    color::WHITE,
    math::{Rect, Vec2},
    text::{TextParams, draw_text_ex, measure_text},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{
    input,
    resource_manager::{ResourceManager, SoundId},
};

const TEXT_SIZE_COEF: f32 = 0.8;
const BTN_SIZE_COEF: f32 = 0.1;
const POS_COEF: f32 = 0.015;

pub trait DrawFn {
    fn draw(&self, x: f32, y: f32, size: f32, is_clicked: bool, resource_manager: &ResourceManager);
}

#[derive(Debug)]
pub struct Button<F: DrawFn> {
    x: f32,
    y: f32,
    size: f32,
    pub is_clicked: bool,
    draw_fn: F,
}
impl<F: DrawFn> Button<F> {
    pub fn new(x: f32, y: f32, size: f32, draw_fn: F) -> Self {
        Self {
            x,
            y,
            size,
            is_clicked: false,
            draw_fn,
        }
    }

    pub fn draw(&mut self, mouse_pos: Vec2, resource_manager: &ResourceManager) {
        let is_hovered = Rect::new(self.x, self.y, self.size, self.size).contains(mouse_pos);
        let texture = if is_hovered {
            &resource_manager.level_button_selected
        } else {
            &resource_manager.level_button
        };
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(self.size)),
                ..Default::default()
            },
        );

        let is_pressed = is_hovered && input::click();
        if is_pressed {
            resource_manager.play_sound(SoundId::Button);
            self.is_clicked = true
        }

        self.draw_fn
            .draw(self.x, self.y, self.size, self.is_clicked, resource_manager);
    }
}

pub fn create_level_button(
    level: usize,
    x: f32,
    y: f32,
    size: f32,
    draw_checkmark: bool,
) -> Button<impl DrawFn> {
    const CHECKMARK_SIZE_OF_BUTTON: f32 = 0.45;
    const CHECKMARK_OFFSET_COEF: f32 = 0.2;
    struct LevelTextDrawFn {
        level: usize,
        draw_checkmark: bool,
    }
    impl DrawFn for LevelTextDrawFn {
        fn draw(
            &self,
            x: f32,
            y: f32,
            size: f32,
            _is_clicked: bool,
            resource_manager: &ResourceManager,
        ) {
            let font_size = (size * TEXT_SIZE_COEF) as u16;

            let text = format!("{}", self.level);
            let text_dimensions = measure_text(&text, Some(&resource_manager.font), font_size, 1.0);
            let margin_x = text_dimensions.width * 0.2;
            let margin_y = text_dimensions.height * 1.2;
            let checkmark_size = size * CHECKMARK_SIZE_OF_BUTTON;
            let checkmark_offset = checkmark_size * CHECKMARK_OFFSET_COEF;

            draw_text_ex(
                &text,
                x + margin_x,
                y + margin_y,
                TextParams {
                    font: Some(&resource_manager.font),
                    color: WHITE,
                    font_size,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
            if self.draw_checkmark {
                draw_texture_ex(
                    &resource_manager.checkmark,
                    x + size - checkmark_size + checkmark_offset,
                    y - checkmark_offset,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::splat(checkmark_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    let draw_fn = LevelTextDrawFn {
        level,
        draw_checkmark,
    };

    Button::new(x, y, size, draw_fn)
}

pub fn create_back_button(screen_width: f32, screen_height: f32) -> Button<impl DrawFn> {
    let button_size = BTN_SIZE_COEF * screen_height;
    let smaller_size = screen_width.min(screen_height);
    let xy = POS_COEF * smaller_size;

    struct BackButtonDrawFn;
    impl DrawFn for BackButtonDrawFn {
        fn draw(
            &self,
            x: f32,
            y: f32,
            size: f32,
            _is_clicked: bool,
            resource_manager: &ResourceManager,
        ) {
            draw_texture_ex(
                &resource_manager.back_arrow,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(size)),
                    ..Default::default()
                },
            );
        }
    }
    let draw_fn = BackButtonDrawFn;

    Button::new(xy, xy, button_size, draw_fn)
}

pub fn create_help_button(screen_width: f32, screen_height: f32) -> Button<impl DrawFn> {
    let button_size = BTN_SIZE_COEF * screen_height;
    let smaller_size = screen_width.min(screen_height);
    let xy = POS_COEF * smaller_size;

    struct HelpButtonDrawFn;
    impl DrawFn for HelpButtonDrawFn {
        fn draw(
            &self,
            x: f32,
            y: f32,
            size: f32,
            _is_clicked: bool,
            resource_manager: &ResourceManager,
        ) {
            let text = " ?";
            let font_size = (size * TEXT_SIZE_COEF) as u16;

            let text_dimensions = measure_text(text, Some(&resource_manager.font), font_size, 1.0);
            let margin_x = text_dimensions.width * 0.2;
            let margin_y = text_dimensions.height * 1.2;

            draw_text_ex(
                text,
                x + margin_x,
                y + margin_y,
                TextParams {
                    font: Some(&resource_manager.font),
                    color: WHITE,
                    font_size,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
        }
    }
    let draw_fn = HelpButtonDrawFn;

    Button::new(xy, xy, button_size, draw_fn)
}
