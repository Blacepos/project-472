use super::flatten::Flatten;


pub struct Light {
    pub position: (f32, f32, f32),
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32)
}

impl Light {
    pub fn add_to_buffer(self, buf: &mut Vec<f32>) {
        buf.extend(self.flatten())
    }
}

impl Flatten for Light {
    fn flatten(self) -> Vec<f32> {
        vec![
            self.position.0,
            self.position.1,
            self.position.2,
            self.ambient.0,
            self.ambient.1,
            self.ambient.2,
            self.diffuse.0,
            self.diffuse.1,
            self.diffuse.2,
            self.specular.0,
            self.specular.1,
            self.specular.2,
        ]
    }
}
