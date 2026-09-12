use macroquad::{
    math::{Vec2, Vec4, vec2, vec4},
    prelude::{
        Material, MaterialParams, ShaderSource, UniformDesc, UniformType, gl_use_material,
        load_material,
    },
};

use crate::level::{AboveTile, FloorTile};

const VERTEX_SHADER: &str = include_str!("../../resources/shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../resources/shaders/fragment.glsl");

const MAX_LIGHTS_COUNT: usize = 32;
const LIGHT_POSITIONS_UNIFORM: &str = "lightPositions";
const LIGHT_COLORS_UNIFORM: &str = "lightColors";
const LIGHTS_COUNT_UNIFORM: &str = "lightsCount";
const ASPECT_RATIO_UNIFORM: &str = "aspectRatio";

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
        let aspect_ratio_uniform = UniformDesc::new(ASPECT_RATIO_UNIFORM, UniformType::Float1);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                pipeline_params: Default::default(),
                uniforms: vec![
                    light_positions_uniform,
                    light_colors_uniform,
                    lights_count_uniform,
                    aspect_ratio_uniform,
                ],
                ..Default::default()
            },
        )
        .expect("Error initialising shaders");

        Self { material }
    }

    pub fn set_shader(&self, lights: &[Light], screen_width: f32, screen_height: f32) {
        let aspect_ratio = screen_width / screen_height;
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
            .set_uniform(ASPECT_RATIO_UNIFORM, aspect_ratio);

        gl_use_material(&self.material);
    }
}
