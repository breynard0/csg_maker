use glam::Vec3;

use super::hierarchy::Hierarchy;

pub struct LogicalObject {
    pub name: String,
    pub id: u32,
    pub origin: glam::Vec3,
    pub rotation: glam::Vec3,
    pub scale: glam::Vec3,
}

impl LogicalObject {
    pub fn new_empty(hierarchy: &Hierarchy) -> Self {
        let list = &hierarchy.flat_list;
        let mut id = list.len() as u32;
        while !list.iter().find(|x| x.id == id).is_some() {
            id += 1;
        }

        LogicalObject {
            name: LogicalObject::check_name(hierarchy, "NewObject".to_string()),
            id,
            origin: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    fn check_name(hierarchy: &Hierarchy, tentative_name: String) -> String {
        let mut tentative_num = 0;
        let mut name = format!("{}.{}", tentative_name, tentative_num);
        while hierarchy.flat_list.iter().find(|x| x.name == name).is_some() {
            tentative_num += 1;
            name = format!("{}.{}", tentative_name, tentative_num);
        }

        if tentative_num == 0 {
            name = tentative_name;
        }

        name
    }
}