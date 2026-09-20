use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::WHITE,
    math::{Rect, Vec2, Vec4, vec2, vec4},
    prelude::{
        Material, MaterialParams, ShaderSource, UniformDesc, UniformType, gl_use_default_material,
        gl_use_material, load_material,
    },
    texture::{draw_texture, render_target},
    window::clear_background,
};

use crate::level::{AboveTile, FloorTile};

const VERTEX_SHADER: &str = include_str!("../../resources/shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../resources/shaders/fragment.glsl");

const MAX_LIGHTS_COUNT: usize = 32;
const LIGHT_POSITIONS_UNIFORM: &str = "lightPositions";
const LIGHT_COLORS_UNIFORM: &str = "lightColors";
const LIGHTS_COUNT_UNIFORM: &str = "lightsCount";
const SCREEN_WIDTH_UNIFORM: &str = "screenWidth";
const SCREEN_HEIGHT_UNIFORM: &str = "screenHeight";

#[derive(Debug, Clone, Copy)]
pub struct Light {
    position: Vec2,
    color: Vec4,
}
impl Light {
    pub fn from_tile(tile: AboveTile, pos: Vec2) -> Option<Light> {
        match tile {
            AboveTile::Player => Some(Light {
                color: vec4(0.7, 1.0, 0.7, 0.09),
                position: pos,
            }),
            _ => None,
        }
    }

    pub fn from_floor(tile: FloorTile, pos: Vec2) -> Option<Light> {
        match tile {
            FloorTile::PlayerExit => Some(Light {
                color: vec4(0.7, 1.0, 0.7, 0.08),
                position: pos,
            }),
            FloorTile::BoxExit => Some(Light {
                color: vec4(1.0, 1.0, 0.4, 0.08),
                position: pos,
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    material: Material,
}
impl Shader {
    pub fn new() -> Self {
        let light_positions_uniform =
            UniformDesc::new(LIGHT_POSITIONS_UNIFORM, UniformType::Float2).array(MAX_LIGHTS_COUNT);
        let light_colors_uniform =
            UniformDesc::new(LIGHT_COLORS_UNIFORM, UniformType::Float4).array(MAX_LIGHTS_COUNT);
        let lights_count_uniform = UniformDesc::new(LIGHTS_COUNT_UNIFORM, UniformType::Int1);
        let screen_width_uniform = UniformDesc::new(SCREEN_WIDTH_UNIFORM, UniformType::Float1);
        let screen_height_uniform = UniformDesc::new(SCREEN_HEIGHT_UNIFORM, UniformType::Float1);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    light_positions_uniform,
                    light_colors_uniform,
                    lights_count_uniform,
                    screen_width_uniform,
                    screen_height_uniform,
                ],
                ..Default::default()
            },
        )
        .expect("Error initialising shaders");

        Self { material }
    }

    pub fn use_shader<F>(&self, screen_width: f32, screen_height: f32, draw_fn: F)
    where
        F: Fn() -> Vec<Light>,
    {
        let render_target = render_target(screen_width as u32, screen_height as u32);
        render_target
            .texture
            .set_filter(macroquad::texture::FilterMode::Nearest);
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width, screen_height));
        camera.render_target = Some(render_target.clone());
        set_camera(&camera);

        let lights = draw_fn();

        set_default_camera();
        clear_background(WHITE);
        self.set_shader(&lights, screen_width, screen_height);
        draw_texture(&render_target.texture, 0.0, 0.0, WHITE);
        gl_use_default_material();
    }

    fn set_shader(&self, lights: &[Light], screen_width: f32, screen_height: f32) {
        let lights_count = lights.len().min(MAX_LIGHTS_COUNT) as i32;
        let mut light_color_arr: [Vec4; MAX_LIGHTS_COUNT] = Default::default();
        let mut light_pos_arr: [Vec2; MAX_LIGHTS_COUNT] = Default::default();

        self.material
            .set_uniform(LIGHTS_COUNT_UNIFORM, lights_count);
        for (i, light) in lights.iter().take(MAX_LIGHTS_COUNT).enumerate() {
            light_pos_arr[i] = light.position / vec2(screen_width, screen_height);
            light_pos_arr[i].y = 1.0 - light_pos_arr[i].y;
            light_color_arr[i] = light.color;
        }

        self.material
            .set_uniform_array(LIGHT_COLORS_UNIFORM, &light_color_arr);
        self.material
            .set_uniform_array(LIGHT_POSITIONS_UNIFORM, &light_pos_arr);
        self.material
            .set_uniform(SCREEN_WIDTH_UNIFORM, screen_width);
        self.material
            .set_uniform(SCREEN_HEIGHT_UNIFORM, screen_height);

        gl_use_material(&self.material);
    }
}
