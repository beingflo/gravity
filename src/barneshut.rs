use nannou::prelude::*;

struct Quadrants {
    quadrants: (Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>, Box<BarnesHut>),
    top_left: Vector2,
    bottom_right: Vector2,
}

enum Node {
    Children(Quadrants),
    Single(Vector2),
    Empty,
}


pub struct BarnesHut {
    child: Node,
}

impl BarnesHut {
    pub fn new(points: &[Vector2]) -> Self {
        let mut tree = BarnesHut { child: Node::Empty };

        for p in points {
            seep(&mut tree, *p);
        }

        tree
    }
}

fn seep(tree: &mut BarnesHut, elem: Vector2) {
    if let Node::Single(leaf) = tree.child {
        let up = leaf.y.max(elem.y);
        let down = leaf.y.min(elem.y);
        let left = leaf.x.min(elem.x);
        let right = leaf.x.max(elem.x);
    } else {
        seep(tree, elem);
    }
}
