#[derive(Debug, Clone)]
pub struct Color3f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color3f {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {r, g, b}
    }

    pub fn splat(v: f32) -> Self {
        Self {
            r: v,
            g: v,
            b: v
        }
    }
}

impl Color4f {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {r, g, b, a}
    }
}