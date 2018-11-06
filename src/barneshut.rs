use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;
use camera::Camera;

enum Node {
    Quadrants(Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>),
    Single(Vector2),
    Empty,
}


pub struct BarnesHut {
    children: Node,
    upper_left: Vector2,
    lower_right: Vector2,

    // Center of mass
    com: Vector2,
    // Number of points
    n: usize,
}

impl BarnesHut {
    pub fn new(points: &[Particle]) -> Self {
        let mut tree = BarnesHut { children: Node::Empty, upper_left: Vector2::new(0.0, 0.0), lower_right: Vector2::new(0.0, 0.0), com: Vector2::new(0.0, 0.0), n: points.len() };

        let mut x_max = points[0].pos.x;
        let mut x_min = points[0].pos.x;
        let mut y_max = points[0].pos.y;
        let mut y_min = points[0].pos.y;

        for p in points {
            x_max = p.pos.x.max(x_max);
            x_min = p.pos.x.min(x_min);
            y_max = p.pos.y.max(y_max);
            y_min = p.pos.y.min(y_min);
        }

        tree.upper_left = Vector2::new(x_min, y_max);
        tree.lower_right = Vector2::new(x_max, y_min);

        tree
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera) {
        draw_rectangle(draw, camera, self.upper_left, self.lower_right);
    }
}

fn draw_rectangle(draw: &Draw, camera: &Camera, upper_left: Vector2, lower_right: Vector2) {
    let upper_right = Vector2::new(lower_right.x, upper_left.y);
    let lower_left = Vector2::new(upper_left.x, lower_right.y);

    let ul_rel = (upper_left - camera.lookat) * camera.zoom;
    let ur_rel = (upper_right - camera.lookat) * camera.zoom;
    let ll_rel = (lower_left - camera.lookat) * camera.zoom;
    let lr_rel = (lower_right - camera.lookat) * camera.zoom;

    let thickness = (5.0 * camera.zoom).max(2.0);

    draw.line().start(ul_rel).end(ur_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(ur_rel).end(lr_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(lr_rel).end(ll_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(ll_rel).end(ul_rel).thickness(thickness).caps_round().color(GREEN);
}
