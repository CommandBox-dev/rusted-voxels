use std::iter::Once;

use fastnoise_lite::FractalType::PingPong;
use glam::Vec3;

use crate::engine::animation::{animation::{Animation, AnimationTrack, Bone, PlaybackMode, QuatKeyframe, VecKeyframe}, skeleton::Skeleton};

pub struct AnimationPlayer {
    delta: f32,
    animations: Vec<Animation>,
}

pub struct PlayingAnimation {
    pub animation_id: u32,
    pub reversed: bool,
    pub speed: f32,
    pub time: f32,
    pub playing: bool,
}

impl AnimationPlayer { // animation manager
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            animations: Vec::new(),
        }
    }

    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    pub fn update(&mut self, delta: f32) {
        self.delta = delta;
    }

    pub fn update_animation(&self, playing_animation: &mut PlayingAnimation, skeleton: &mut Skeleton) {

        // IF ONE TRACK HAS LESS THAN TWO KEYFRAMES IT WON'T WORK

        playing_animation.time += self.delta * playing_animation.speed;

        let animation = &self.animations.get(playing_animation.animation_id as usize);
        if animation.is_none() {
            println!("Animation '#{}' not found!", playing_animation.animation_id);
            return;
        }
        let animation = animation.unwrap();

        let duration = animation.duration;

        for bone_id in 0..animation.tracks.len() {

            let bone = skeleton.bones.get_mut(bone_id);
            if bone.is_none() {return;}
            let bone = bone.unwrap();

            let bone_track = &animation.tracks[bone_id];

            let position_timeline = &bone_track.position;
            let rotation_timeline = &bone_track.rotation;
            let scale_timeline = &bone_track.scale;

            // Position
            let mut end_pos = true;
            let mut last_keyframe = &VecKeyframe::new(Vec3::ZERO, 0.0);
            for keyframe in position_timeline {

                if keyframe.time > playing_animation.time
                && last_keyframe.time <= playing_animation.time {

                    let local_time = (playing_animation.time - last_keyframe.time) / (keyframe.time - last_keyframe.time);
                    VecKeyframe::interpolate(bone, last_keyframe, keyframe, local_time, true);
                    end_pos = false;
                    break;
                }
                last_keyframe = keyframe;
            }
            if end_pos {VecKeyframe::interpolate(bone, last_keyframe, last_keyframe, 1.0, true);}

            // Rotation
            let mut end_rot = true;
            let mut last_keyframe = &QuatKeyframe::from_quat(bone.rotation, 0.0);
            for keyframe in rotation_timeline {
                
                if keyframe.time > playing_animation.time
                && last_keyframe.time <= playing_animation.time {
                    
                    let local_time = (playing_animation.time - last_keyframe.time) / (keyframe.time - last_keyframe.time);
                    QuatKeyframe::interpolate(bone, last_keyframe, keyframe, local_time);
                    end_rot = false;
                    break;
                }
                last_keyframe = keyframe;
            }

            // Scale
            let mut end_scale = true;
            let mut last_keyframe = &VecKeyframe::new(Vec3::ONE, 0.0);
            for keyframe in scale_timeline {
                
                if keyframe.time > playing_animation.time
                && last_keyframe.time <= playing_animation.time {
                    
                    let local_time = (playing_animation.time - last_keyframe.time) / (keyframe.time - last_keyframe.time);
                    VecKeyframe::interpolate(bone, last_keyframe, keyframe, local_time, false);
                    end_scale = false;
                    break;
                }
                last_keyframe = keyframe;
            }

            if end_pos && end_rot && end_scale {
                match animation.playback_mode {
                    PlaybackMode::Once => {}
                    PlaybackMode::Loop => {playing_animation.time = 0.0}
                    PlaybackMode::PingPong => {}
                }
            }

            bone.update_transform();
        }
    }
}