use nannou::prelude::*;
use nannou::draw::Draw;

use particle::Particle;
use camera::Camera;

#[derive(Copy, Clone)]
enum Quadrant { UL, UR, LL, LR }

pub struct Node {
    upper_left: Vector2,
    lower_right: Vector2,

    // Center of mass
    com: Vector2,
    // Number of points
    n: usize,

    children: Option<Box<[Node; 4]>>,
}

impl Node {
    pub fn new(upper_left: Vector2, lower_right: Vector2) -> Self {
        Node { upper_left: upper_left, lower_right: lower_right, com: Vector2::new(0.0, 0.0), n: 0, children: None }
    }

    fn insert(&mut self, point: Vector2) {
        if self.children.is_some() {
            self.add_mass(point);
            let quadrant = self.get_quadrant(point);
            if let Some(ref mut children) = self.children {
                children[quadrant as usize].insert(point);
            }
        } else {
            if self.n == 0 {
                self.add_mass(point);
            } else {
                let old_com = self.com;

                self.com = Vector2::new(0.0, 0.0);
                self.n = 0;

                self.create_children();
                self.insert(old_com);
                self.insert(point);
            }

        }
    }

    pub fn compute_force(&self, point: Vector2, theta: f32) -> Vector2 {
        let mut force = Vector2::new(0.0, 0.0);

        if let Some(ref children) = self.children {
            let side = self.upper_left.y - self.lower_right.y;
            let diff = self.com - point;
            let d = (diff.x * diff.x + diff.y * diff.y).sqrt();

            if side / d < theta {
                // Approximate
                force += pair_force(point, self.com) * self.n as f32;
            } else {
                // Recurse
                for q in children.iter() {
                    force += q.compute_force(point, theta);
                }
            }
        } else {
            force += pair_force(point, self.com);
        }

        force
    }

    fn create_children(&mut self) {
        assert!(self.children.is_none());

        let mid_x = (self.upper_left.x + self.lower_right.x) / 2.0;
        let mid_y = (self.upper_left.y + self.lower_right.y) / 2.0;

        self.children = Some(Box::new([
            Node::new(self.upper_left, Vector2::new(mid_x, mid_y)), // UL
            Node::new(Vector2::new(mid_x, self.upper_left.y), Vector2::new(self.lower_right.x, mid_y)), // UR
            Node::new(Vector2::new(self.upper_left.x, mid_y), Vector2::new(mid_x, self.lower_right.y)), // LL
            Node::new(Vector2::new(mid_x, mid_y), self.lower_right)  // LR
        ]));

    }

    fn get_quadrant(&self, point: Vector2) -> Quadrant {
        let mid_x = (self.upper_left.x + self.lower_right.x) / 2.0;
        let mid_y = (self.upper_left.y + self.lower_right.y) / 2.0;

        if point.x > mid_x {
            if point.y > mid_y {
                Quadrant::UR
            } else {
                Quadrant::LR
            }
        } else {
            if point.y > mid_y {
                Quadrant::UL
            } else {
                Quadrant::LL
            }
        }
    }

    fn add_mass(&mut self, point: Vector2) {
        if self.n == 0 {
            self.com = point;
            self.n += 1;
        } else {
            self.com = (self.com * self.n as f32 + point) / (self.n as f32 + 1.0);
            self.n += 1;
        }
    }

    pub fn draw(&self, draw: &Draw, camera: &Camera) {
        draw_rectangle(draw, camera, self.upper_left, self.lower_right);

        if let Some(ref children) = self.children {
            for q in children.iter() {
                q.draw(draw, camera);
            }
        }
    }
}

pub fn construct_tree(points: &[Particle]) -> Node {
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

    let mut tree = Node::new(Vector2::new(x_min, y_max), Vector2::new(x_max, y_min));

    for p in points {
        tree.insert(p.pos);
    }

    tree
}

fn pair_force(a: Vector2, b: Vector2) -> Vector2 {
    let mut force = Vector2::new(0.0, 0.0);

    // Not realistic
    let g = 5000.0;

    let eps = 5.0;

    //if a.x == b.x && a.y == b.y {
    //    return force;
    //}

    let diff = a - b;
    let r2 = (diff.x * diff.x) + (diff.y * diff.y);
    force = -(diff / (r2 + eps).powf(3.0 / 2.0)) * g;

    force
}

fn draw_rectangle(draw: &Draw, camera: &Camera, upper_left: Vector2, lower_right: Vector2) {
    let upper_right = Vector2::new(lower_right.x, upper_left.y);
    let lower_left = Vector2::new(upper_left.x, lower_right.y);

    let ul_rel = (upper_left - camera.lookat) * camera.zoom;
    let ur_rel = (upper_right - camera.lookat) * camera.zoom;
    let ll_rel = (lower_left - camera.lookat) * camera.zoom;
    let lr_rel = (lower_right - camera.lookat) * camera.zoom;

    let thickness = (2.0 * camera.zoom).max(1.0);

    draw.line().start(ul_rel).end(ur_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(ur_rel).end(lr_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(lr_rel).end(ll_rel).thickness(thickness).caps_round().color(GREEN);
    draw.line().start(ll_rel).end(ul_rel).thickness(thickness).caps_round().color(GREEN);
}
