use macroquad::{
    color::{Color, WHITE},
    math::{Rect, Vec2},
    prelude::warn,
    shapes::{DrawRectangleParams, draw_rectangle_lines_ex},
    text::{TextParams, draw_text_ex, measure_text},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

use crate::{
    input::{self, MovementInput},
    resource_manager::{ResourceManager, SoundId},
    shared_state::SharedState,
};

const TEXT_SIZE_COEF: f32 = 0.8;
const POS_COEF: f32 = 0.015;

#[derive(Debug, Clone)]
enum ButtonType {
    Level { level: usize, draw_checkmark: bool },
    Back,
    Help,
    Undo,
    Reset,
    SoundOn,
    SoundOff,
    FullscreenOn,
    FullscreenOff,
}

#[derive(Debug, Clone, Copy)]
enum ButtonPosition {
    Left(u8),
    Right(u8),
}

#[derive(Debug)]
pub struct Button {
    x: f32,
    y: f32,
    size: f32,
    pub is_clicked: bool,
    is_hovered: bool,
    button_type: ButtonType,
}
impl Button {
    pub fn new_level(
        level_number: usize,
        x: f32,
        y: f32,
        button_size: f32,
        draw_checkmark: bool,
    ) -> Self {
        Self::new(
            x,
            y,
            button_size,
            ButtonType::Level {
                level: level_number,
                draw_checkmark,
            },
        )
    }

    pub fn new_help(screen_width: f32, screen_height: f32, is_mobile: bool) -> Self {
        let button_size = Self::get_button_size_coef(is_mobile) * screen_height;
        let smaller_size = screen_width.min(screen_height);
        let xy = POS_COEF * smaller_size;

        Self::new(xy, xy, button_size, ButtonType::Help)
    }

    pub fn new_back(screen_width: f32, screen_height: f32, is_mobile: bool) -> Self {
        Self::new_image_button(
            screen_width,
            screen_height,
            ButtonPosition::Left(0),
            ButtonType::Back,
            is_mobile,
        )
    }

    pub fn new_undo(screen_width: f32, screen_height: f32, is_mobile: bool) -> Self {
        Self::new_image_button(
            screen_width,
            screen_height,
            ButtonPosition::Right(0),
            ButtonType::Undo,
            is_mobile,
        )
    }

    pub fn new_reset(screen_width: f32, screen_height: f32, is_mobile: bool) -> Self {
        Self::new_image_button(
            screen_width,
            screen_height,
            ButtonPosition::Left(1),
            ButtonType::Reset,
            is_mobile,
        )
    }

    pub fn new_sound(screen_width: f32, screen_height: f32, shared_state: &SharedState) -> Self {
        let button_type = if shared_state.is_sound_on {
            ButtonType::SoundOn
        } else {
            ButtonType::SoundOff
        };

        Self::new_image_button(
            screen_width,
            screen_height,
            ButtonPosition::Left(1),
            button_type,
            shared_state.is_mobile,
        )
    }

    pub fn new_fullscreen(
        screen_width: f32,
        screen_height: f32,
        is_fullscreen: bool,
        is_mobile: bool,
    ) -> Self {
        let button_type: ButtonType = if is_fullscreen {
            ButtonType::FullscreenOff
        } else {
            ButtonType::FullscreenOn
        };

        Self::new_image_button(
            screen_width,
            screen_height,
            ButtonPosition::Left(2),
            button_type,
            is_mobile,
        )
    }

    fn new_image_button(
        screen_width: f32,
        screen_height: f32,
        position: ButtonPosition,
        button_type: ButtonType,
        is_mobile: bool,
    ) -> Self {
        let button_size = Self::get_button_size_coef(is_mobile) * screen_height;
        let smaller_size = screen_width.min(screen_height);
        let margin = POS_COEF * smaller_size;

        let (x, y) = match position {
            ButtonPosition::Left(y) => (margin, margin + (button_size + margin) * y as f32),
            ButtonPosition::Right(y) => (
                screen_width - (margin + button_size),
                margin + (button_size + margin) * y as f32,
            ),
        };

        Self::new(x, y, button_size, button_type)
    }

    fn new(x: f32, y: f32, size: f32, button_type: ButtonType) -> Self {
        Self {
            x,
            y,
            size,
            is_clicked: false,
            button_type,
            is_hovered: false,
        }
    }

    pub fn detect(&mut self, mouse_vec: Vec2, shared_state: &SharedState) {
        self.is_hovered = Rect::new(self.x, self.y, self.size, self.size).contains(mouse_vec);
        let is_pressed = self.is_hovered && input::click();
        if is_pressed {
            shared_state.play_sound_if_enabled(SoundId::Button);
            self.is_clicked = true
        }
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        let texture = if self.is_hovered {
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

        match self.button_type {
            ButtonType::Level {
                level,
                draw_checkmark,
            } => self.draw_level_button(draw_checkmark, level, resource_manager),
            ButtonType::Help => self.draw_help_button(resource_manager),
            ButtonType::Back => self.draw_texture_button(&resource_manager.back_arrow),
            ButtonType::Undo => self.draw_texture_button(&resource_manager.undo_arrow),
            ButtonType::Reset => self.draw_texture_button(&resource_manager.reset_x),
            ButtonType::SoundOn => self.draw_texture_button(&resource_manager.sound_on),
            ButtonType::SoundOff => self.draw_texture_button(&resource_manager.sound_off),
            ButtonType::FullscreenOn => {
                self.draw_texture_button(&resource_manager.fullscreen_on_arrows)
            }
            ButtonType::FullscreenOff => {
                self.draw_texture_button(&resource_manager.fullscreen_off_arrows)
            }
        }
    }

    fn draw_texture_button(&self, texture: &Texture2D) {
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
    }

    fn draw_level_button(
        &self,
        draw_checkmark: bool,
        level: usize,
        resource_manager: &ResourceManager,
    ) {
        const CHECKMARK_SIZE_OF_BUTTON: f32 = 0.45;
        const CHECKMARK_OFFSET_COEF: f32 = 0.2;

        let font_size = (self.size * TEXT_SIZE_COEF) as u16;

        let text = format!("{}", level);
        let text_dimensions = measure_text(&text, Some(&resource_manager.font), font_size, 1.0);
        let margin_x = text_dimensions.width * 0.2;
        let margin_y = text_dimensions.height * 1.2;
        let checkmark_size = self.size * CHECKMARK_SIZE_OF_BUTTON;
        let checkmark_offset = checkmark_size * CHECKMARK_OFFSET_COEF;

        draw_text_ex(
            &text,
            self.x + margin_x,
            self.y + margin_y,
            TextParams {
                font: Some(&resource_manager.font),
                color: WHITE,
                font_size,
                font_scale: 1.0,
                ..Default::default()
            },
        );
        if draw_checkmark {
            draw_texture_ex(
                &resource_manager.checkmark,
                self.x + self.size - checkmark_size + checkmark_offset,
                self.y - checkmark_offset,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(checkmark_size)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_help_button(&self, resource_manager: &ResourceManager) {
        let text = " ?";
        let font_size = (self.size * TEXT_SIZE_COEF) as u16;

        let text_dimensions = measure_text(text, Some(&resource_manager.font), font_size, 1.0);
        let margin_x = text_dimensions.width * 0.2;
        let margin_y = text_dimensions.height * 1.2;

        draw_text_ex(
            text,
            self.x + margin_x,
            self.y + margin_y,
            TextParams {
                font: Some(&resource_manager.font),
                color: WHITE,
                font_size,
                font_scale: 1.0,
                ..Default::default()
            },
        );
    }

    fn get_button_size_coef(is_mobile: bool) -> f32 {
        const DESKTOP_BTN_SIZE_COEF: f32 = 0.13;
        const MOBILE_BTN_SIZE_COEF: f32 = 0.18;

        if is_mobile {
            MOBILE_BTN_SIZE_COEF
        } else {
            DESKTOP_BTN_SIZE_COEF
        }
    }
}

#[derive(Debug)]
pub struct MobileMovementButtons {
    buttons: [(MovementInput, Rect); 4],
    hovered: [bool; 4],
}
impl MobileMovementButtons {
    const SINGLE_BUTTON_SIZE_COEF: f32 = 0.2;
    const MARGIN_COEF: f32 = 0.015;
    const INNER_MARGIN: f32 = 4.0;

    pub fn new(width: f32, height: f32) -> Self {
        let size = Self::SINGLE_BUTTON_SIZE_COEF * height;
        let margin = Self::MARGIN_COEF * width.min(height);

        let right_x = width - margin - size;
        let middle_x = right_x - size - Self::INNER_MARGIN;
        let left_x = middle_x - size - Self::INNER_MARGIN * 2.0;

        let bottom_y = height - margin;
        let down_y = bottom_y - size;
        let up_y = bottom_y - size * 2.0 - Self::INNER_MARGIN;
        let side_y = bottom_y - size * 1.5;

        Self {
            buttons: [
                (MovementInput::Up, Rect::new(middle_x, up_y, size, size)),
                (MovementInput::Down, Rect::new(middle_x, down_y, size, size)),
                (MovementInput::Left, Rect::new(left_x, side_y, size, size)),
                (MovementInput::Right, Rect::new(right_x, side_y, size, size)),
            ],
            hovered: [false; 4],
        }
    }

    pub fn detect(&mut self, mouse_vec: Vec2) -> Option<MovementInput> {
        let clicked = input::click();
        let mut result = None;

        for (i, (movement, rect)) in self.buttons.iter().enumerate() {
            self.hovered[i] = rect.contains(mouse_vec);
            if self.hovered[i] && clicked {
                result = Some(*movement);
            }
        }

        result
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        const BORDER_SIZE: f32 = 4.0;
        const BUTTON_TRASNPARENCY: Color = Color::from_rgba(255, 255, 255, 70);
        const BORDER_COLOR: Color = Color::from_rgba(255, 255, 255, 100);

        for (movement, rect) in &self.buttons {
            draw_rectangle_lines_ex(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                BORDER_SIZE,
                DrawRectangleParams {
                    color: BORDER_COLOR,
                    ..Default::default()
                },
            );

            let arrow_texture = resource_manager.get_arrow_button_texture_for_direction(*movement);
            draw_texture_ex(
                arrow_texture,
                rect.x,
                rect.y,
                BUTTON_TRASNPARENCY,
                DrawTextureParams {
                    dest_size: Some(rect.size()),
                    ..Default::default()
                },
            );
        }
    }
}

#[derive(Debug)]
pub struct MobileButtonsGroup {
    pub undo_button: Button,
    pub reset_button: Button,
    pub mobile_movement_buttons: MobileMovementButtons,
}
impl MobileButtonsGroup {
    pub fn new(width: f32, height: f32, is_mobile: bool) -> Self {
        let undo_button = Button::new_undo(width, height, is_mobile);
        let reset_button = Button::new_reset(width, height, is_mobile);
        let mobile_movement_buttons = MobileMovementButtons::new(width, height);

        Self {
            undo_button,
            reset_button,
            mobile_movement_buttons,
        }
    }

    pub fn detect(&mut self, mouse_vec: Vec2, shared_state: &SharedState) -> Option<MovementInput> {
        if !shared_state.is_mobile {
            warn!("Calling 'detect' on mobile buttons when device isn't mobile");
        }

        self.undo_button.detect(mouse_vec, shared_state);
        self.reset_button.detect(mouse_vec, shared_state);
        let direction_input = self.mobile_movement_buttons.detect(mouse_vec);

        if self.undo_button.is_clicked {
            Some(MovementInput::Undo)
        } else if self.reset_button.is_clicked {
            Some(MovementInput::Reset)
        } else {
            direction_input
        }
    }

    pub fn draw(&self, resource_manager: &ResourceManager) {
        self.undo_button.draw(resource_manager);
        self.reset_button.draw(resource_manager);
        self.mobile_movement_buttons.draw(resource_manager);
    }
}
