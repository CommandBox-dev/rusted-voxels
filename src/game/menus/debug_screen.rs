use crate::engine::{entities::character_controller::CharacterState, gui::{basic_ui::{UIAnchor::{AnchorBottomLeft, AnchorTopLeft}, UIElement, UITransformC}, elements::label::UILabel}, physics::aabb::AABB, resources::resource_manager::ResourceManager, world::world::World};

pub struct DebugScreen {
    version_label: UILabel,
    coord_label: UILabel,
    chunk_info_label: UILabel,
}

impl DebugScreen {
    pub fn new(window_transform: &UITransformC) -> Self {
        Self {
            version_label: UILabel::new(10, -10, String::from("Rusted-Voxels-v0.01"), AnchorBottomLeft, &window_transform),
            coord_label: UILabel::new(10, 20, String::from("x: 0.000, y: 0.000, z: 0.000"), AnchorTopLeft, &window_transform),
            chunk_info_label: UILabel::new(10, 100, String::from("Chunks Loaded: _"), AnchorTopLeft, &window_transform),
        }
    }

    pub fn update(&mut self, world: Option<&World>, character_controller: Option<&CharacterState>) {

        if let Some(world) = world {
            self.chunk_info_label.set_text(
                format!(
                    "Chunks Generating: {}, Chunks Loaded: {},",
                    world.chunk_manager.chunks_generating,
                    world.chunk_manager.chunks_loaded,
                )
            );
        }

        if let Some(character_controller) = character_controller {
            self.coord_label.set_text(
                format!(
                    "x: {:.3}, y: {:.3}, z: {:.3}",
                    character_controller.position.x,
                    character_controller.position.y + AABB::HUMANOID.min.y,
                    character_controller.position.z
                )
            );
        }
    }

    pub fn render(&self, resource_manager: &ResourceManager, in_world: bool) {
        resource_manager.font_tex.bind(0);

        if in_world {
            self.chunk_info_label.draw();
            self.coord_label.draw();
        }

        self.version_label.draw();
    }

    pub fn realign(&mut self, parent_transform: &UITransformC) {
        self.version_label.realign(parent_transform);
        self.coord_label.realign(parent_transform);
        self.chunk_info_label.realign(parent_transform);
    }
}