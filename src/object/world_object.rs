use super::hierarchy::Hierarchy;

#[derive(Debug, Clone)]
pub struct WorldObject {
    pub hierarchy: Hierarchy,
}

impl WorldObject {
    pub fn new() -> Self {
        WorldObject {
            hierarchy: Hierarchy {
                flat_list: vec![],
            },
        }
    }
}