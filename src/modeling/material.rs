use super::flatten::Flatten;

const SHINY: f32 = 100.;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
    pub reflectivity: (f32, f32, f32),
    pub shininess: f32
}

impl Material {
    #[allow(dead_code)]
    pub fn with_ambient(self, value: (f32, f32, f32)) -> Self {
        Self {
            ambient: value,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_diffuse(self, value: (f32, f32, f32)) -> Self {
        Self {
            diffuse: value,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_white_specular(self, value: f32) -> Self {
        Self {
            specular: (value, value, value), // spamton
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_shininess(self, value: f32) -> Self {
        Self {
            shininess: value,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn make_shiny(self) -> Self {
        Self {
            shininess: SHINY,
            ..self
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: (0., 0., 0.),
            diffuse: (0., 0., 0.),
            specular: (0., 0., 0.),
            reflectivity: (0., 0., 0.),
            shininess: 1.,
        }
    }
}

impl Flatten for Material {
    fn flatten(self) -> Vec<f32> {
        vec![
            self.ambient.0,
            self.ambient.1,
            self.ambient.2,
            self.diffuse.0,
            self.diffuse.1,
            self.diffuse.2,
            self.specular.0,
            self.specular.1,
            self.specular.2,
            self.reflectivity.0,
            self.reflectivity.1,
            self.reflectivity.2,
            self.shininess
        ]
    }
}