use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,

    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,

    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,

    pub fov: f32,
    pub near: f32,
    pub far: f32,

    pub viewport_width: u32,
    pub viewport_height: u32,

    pub view: Mat4,
    pub projection: Mat4,
}

impl Camera {
    pub fn new(position: Vec3, pitch: f32, yaw: f32, viewport_width: u32, viewport_height: u32, fov: f32) -> Self {
        Self {
            position,
            front: -Vec3::Z,
            right: Vec3::X,
            up: Vec3::Y,
            pitch: pitch,
            yaw: yaw,
            roll: 0.0,
            fov,
            near: 0.1,
            far: 100.0,
            viewport_width,
            viewport_height,
            view: glam::camera::rh::view::look_at_mat4(position, position + -Vec3::Z, Vec3::Y),
            projection: glam::camera::rh::proj::directx::perspective(fov.to_radians(), (viewport_width as f32 / viewport_height as f32), 0.1, 100.0)
        }
    }

    pub fn recalculate_view(&mut self) {
        self.view = glam::camera::rh::view::look_at_mat4(self.position, self.position + self.front, self.up);
    }

    pub fn recalculate_vectors(&mut self) {
        // front
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        self.front = Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        ).normalize();

        // right
        self.right = self.front.cross(self.up);
    }
}