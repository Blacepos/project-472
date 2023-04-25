use sfml::graphics::Shader;

use super::{object::Object, flatten::Flatten, lighting::Light};

pub struct Scene {
    scene_buffer: Vec<f32>,
    lights_buffer: Vec<f32>,
    camera_pos: (f32, f32, f32)
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_buffer: Vec::new(),
            lights_buffer: Vec::new(),
            camera_pos: (0., 0., 0.)
        }
    }

    pub fn add_object<M>(&mut self, obj: Object<M>)
    where M: Flatten
    {
        obj.add_to_buffer(&mut self.scene_buffer);
    }

    pub fn add_light(&mut self, light: Light) {
        light.add_to_buffer(&mut self.lights_buffer);
    }

    pub fn set_in_shader(self, shader: &mut Shader) {
        shader.set_uniform_array_float("iScene", &self.scene_buffer);
        shader.set_uniform_int("iNumObjs", self.scene_buffer.len() as i32);

        shader.set_uniform_array_float("iLights", &self.lights_buffer);
        shader.set_uniform_int("iNumLights", self.lights_buffer.len() as i32);

        shader.set_uniform_vec3("iCameraPos", self.camera_pos.into());
    }

    pub fn set_camera_pos(&mut self, pos: (f32, f32, f32)) {
        self.camera_pos = pos;
    }
}