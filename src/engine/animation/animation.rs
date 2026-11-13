  use glam::{FloatExt, Mat4, Quat, Vec3};

use crate::engine::animation::skeleton::Skeleton;

pub struct Bone {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,

    pub pivot_point: Vec3,
    
    pub global_transform: Mat4,
    pub transform: Mat4,
}

#[derive(Debug)]
pub struct VecKeyframe {
    pub value: Vec3,
    pub time: f32,
}

pub struct QuatKeyframe {
    pub value: Quat,
    pub time: f32,
}
/// Bone animation track
pub struct AnimationTrack {
    pub bone_id: u32,
    pub position: Vec<VecKeyframe>,
    pub rotation: Vec<QuatKeyframe>,
    pub scale: Vec<VecKeyframe>,
}

pub struct Animation {
    pub fps: u32,
    pub duration: f32,
    pub playback_mode: PlaybackMode,
    pub tracks: Vec<AnimationTrack>,
}

pub enum PlaybackMode {
    Once,
    Loop,
    PingPong,
}

impl VecKeyframe {
    pub fn new(value: Vec3, time: f32) -> Self {
        Self {
            value,
            time,
        }
    }

    pub fn interpolate(bone: &mut Bone, a: &VecKeyframe, b: &VecKeyframe, t: f32, pos: bool) {
        let x = a.value.x.lerp(b.value.x, t);
        let y = a.value.y.lerp(b.value.y, t);
        let z = a.value.z.lerp(b.value.z, t);

        if pos {
            bone.position.x = x;
            bone.position.y = y;
            bone.position.z = z;
        } else {
            bone.scale.x = x;
            bone.scale.y = y;
            bone.scale.z = z;
        }
    }
}

impl QuatKeyframe {
    pub fn new(value: Vec3, time: f32) -> Self {
        Self {
            value: Quat::from_euler(glam::EulerRot::XYZ, value.x, value.y, value.z),
            time,
        }
    }

    pub fn from_quat(value: Quat, time: f32) -> Self {
        Self {
            value,
            time,
        }
    }

    pub fn interpolate(bone: &mut Bone, a: &QuatKeyframe, b: &QuatKeyframe, t: f32) {
        let x = a.value.x.lerp(b.value.x, t);
        let y = a.value.y.lerp(b.value.y, t);
        let z = a.value.z.lerp(b.value.z, t);

        bone.rotation.x = x;
        bone.rotation.y = y;
        bone.rotation.z = z;
    }
}

impl Bone {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3, custom_pivot: Option<Vec3>) -> Self {
        let rotation = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let mat = Mat4::from_scale_rotation_translation(scale, rotation, position);
        Self {
            position,
            rotation,
            scale,
            pivot_point: if custom_pivot.is_some() {custom_pivot.unwrap()} else {Vec3::ZERO},
            global_transform: mat.clone(),
            transform: mat,
        }
    }

    pub fn update_transform(&mut self) {
        
        self.transform = 
        Mat4::from_translation(self.position)
        * Mat4::from_translation(self.pivot_point)
        * Mat4::from_quat(self.rotation)
        * Mat4::from_scale(self.scale)
        * Mat4::from_translation(-self.pivot_point);


        // if the bone has no parent
        self.global_transform = self.transform;
        // else
        // self.global_transform *= self.transform;
    }
}