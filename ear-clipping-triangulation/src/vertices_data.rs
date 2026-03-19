use eframe::egui;

pub struct VerticesData {
    vertices: Vec<egui::Pos2>,
}

impl VerticesData {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, new_vertex: egui::Pos2) {
        // min distance between consequative points
        let min_distance = 30.0;

        let should_add = match self.vertices.last() {
            Some(&last_pos) => last_pos.distance(new_vertex) > min_distance,
            // always add the first point if empty
            None => true,
        };

        if should_add {
            self.vertices.push(new_vertex);
        }
    }

    pub fn get_len(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_start_end_points(&self, i: usize) -> (egui::Pos2, egui::Pos2) {
        let start_point = self.vertices[i];
        // connect the last point back to the first point to close the shape
        let end_point = self.vertices[(i + 1) % self.vertices.len()];
        (start_point, end_point)
    }

    pub fn pop(&mut self) {
        self.vertices.pop();
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn triangulate(&self) -> Vec<[egui::Pos2; 3]> {
        let n = self.vertices.len();
        if n < 3 {
            return vec![];
        }

        let mut triangles = Vec::new();
        // change the copy so the user input stays unchanged
        let mut v = self.vertices.clone();
        // y goes down positive area is clockwise
        let is_clockwise = is_clockwise(&v);

        // prevent infinite loops if polygon is self-intersecting
        let mut failsafe = v.len() * 2;

        while v.len() > 3 {
            failsafe -= 1;
            if failsafe == 0 {
                // invalid geometry
                break;
            }

            let mut ear_found = false;
            let len = v.len();

            for i in 0..len {
                let prev = v[(i + len - 1) % len];
                let curr = v[i];
                let next = v[(i + 1) % len];

                let cp = cross_product(prev, curr, next);
                let is_convex = if is_clockwise { cp >= 0.0 } else { cp <= 0.0 };

                if !is_convex {
                    continue;
                }

                let mut is_empty = true;
                for j in 0..len {
                    if j == i || j == (i + len - 1) % len || j == (i + 1) % len {
                        continue;
                    }
                    if point_in_triangle(v[j], prev, curr, next) {
                        is_empty = false;
                        break;
                    }
                }

                // valid ear found
                if is_empty {
                    triangles.push([prev, curr, next]);
                    v.remove(i);
                    ear_found = true;
                    break;
                }
            }

            if !ear_found {
                break;
            }
        }

        // add the final triangle
        if v.len() == 3 {
            triangles.push([v[0], v[1], v[2]]);
        }

        triangles
    }
}

// =========== private helpers ===========
fn cross_product(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> f32 {
    let ba_x = b.x - a.x;
    let ba_y = b.y - a.y;
    let cb_x = c.x - b.x;
    let cb_y = c.y - b.y;
    ba_x * cb_y - ba_y * cb_x
}

fn point_in_triangle(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> bool {
    let cp1 = cross_product(a, b, p);
    let cp2 = cross_product(b, c, p);
    let cp3 = cross_product(c, a, p);

    // if the cross products all have the same sign, the point is inside.
    let has_neg = (cp1 < 0.0) || (cp2 < 0.0) || (cp3 < 0.0);
    let has_pos = (cp1 > 0.0) || (cp2 > 0.0) || (cp3 > 0.0);

    !(has_neg && has_pos)
}

fn is_clockwise(vertices: &Vec<egui::Pos2>) -> bool {
    let mut area = 0.0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
    }
    // y goes down positive area is clockwise
    area > 0.0
}
