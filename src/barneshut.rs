use nannou::prelude::*;

enum Node {
    Quadrants(Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>),
    Single(Vector2),
    Empty,
}


pub struct BarnesHut {
    children: Node,
    top_left: Vector2,
    bottom_right: Vector2,

    // Center of mass
    com: Vector2,
    // Number of points
    n: usize,
}

impl BarnesHut {
    pub fn new(points: &[Vector2]) -> Self {
        let mut tree = BarnesHut { children: Node::Empty, top_left: Vector2::new(0.0, 0.0), bottom_right: Vector2::new(0.0, 0.0), com: Vector2::new(0.0, 0.0), n: points.len() };

        let mut x_max = points[0].x;
        let mut x_min = points[0].x;
        let mut y_max = points[0].y;
        let mut y_min = points[0].y;

        for p in points {
            x_max = p.x.max(x_max);
            x_min = p.x.min(x_min);
            y_max = p.y.max(y_max);
            y_min = p.y.min(y_min);
        }

        tree
    }
}
