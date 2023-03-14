use crate::nalgebra_glm;

pub struct Transform{
    position : nalgebra_glm::Vec3,
    rotation : nalgebra_glm::Quat,
    scale : nalgebra_glm::Vec3,
}

impl Transform {
    pub fn new() -> Self{
        Self{
            position : nalgebra_glm::Vec3::new(0f32, 0f32, 0f32),
            rotation: nalgebra_glm::Quat::identity(),
            scale: nalgebra_glm::Vec3::new(1f32,1f32,1f32),
        }
    }
}