use macroquad::prelude::*;

// https://textbooks.cs.ksu.edu/cis580/04-collisions/04-separating-axis-theorem/index.html

#[derive(Default)]
pub struct BoundingPolygon {
    pub corners: Vec<Vec2>,
    normals: Vec<Vec2>,
}

impl BoundingPolygon {
    pub fn update(&mut self, corners: &Vec<Vec2>) {
        self.corners = corners.clone();

        let mut normals = Vec::<Vec2>::new();
        let mut edge = self.corners[self.corners.len() - 1] - self.corners[0];
        let mut perp = edge.perp().normalize_or_zero();
        normals.push(perp);

        for i in 1..self.corners.len() {
            edge = self.corners[i] - self.corners[i - 1];
            perp = edge.perp().normalize_or_zero();
            normals.push(perp);
        }

        self.normals = normals;
    }
}

#[derive(Default)]
struct MinMax {
    min: f32,
    max: f32,
}

impl MinMax {
    fn find_projection(poly: &BoundingPolygon, axis: Vec2) -> Self {
        let mut projection = poly.corners[0].dot(axis);
        let mut max = projection;
        let mut min = projection;
        for i in 1..poly.corners.len() {
            projection = poly.corners[i].dot(axis);
            max = if max > projection { max } else { projection };
            min = if min < projection { min } else { projection };
        }
        MinMax { min, max }
    }
}

fn collides(p1: &BoundingPolygon, p2: &BoundingPolygon) -> bool {
    let poly_normals = [&p1.normals, &p2.normals];
    // Check for each polygon's normals
    for normals in poly_normals {
        for normal in normals {
            // Determine the minimum and maximum projection
            // for both polygons
            let mm1 = MinMax::find_projection(p1, *normal);
            let mm2 = MinMax::find_projection(p2, *normal);

            // Test for seperation (as soon as we find a seperating axis
            // we know there is no possibility of collision, so we can
            // exit early
            if (mm1.max < mm2.min) || (mm2.max < mm1.min) {
                return false;
            }
        }
    }
    // If we reach this point, no seperating axis was found
    // and the two polygons are colliding
    true
}
