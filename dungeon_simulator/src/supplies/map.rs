use std::collections::HashMap;

#[derive(Debug)]
pub struct PointOfInterest {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub description: String,
    pub next_points: Vec<String>,
}

#[derive(Debug)]
pub struct Map {
    points_of_interest: HashMap<String, PointOfInterest>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            points_of_interest: HashMap::new(),
        }
    }

    pub fn add_point_of_interest(&mut self, point: PointOfInterest) {
        self.points_of_interest.insert(point.name.clone(), point);
    }

    pub fn get_point_of_interest(&self, name: &str) -> Option<&PointOfInterest> {
        self.points_of_interest.get(name)
    }
}