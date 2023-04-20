use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use crate::edge::Edge;
use crate::point::Point;
use vecmath::{vec3_cross, vec3_dot, vec3_len, Vector3};

pub fn calc_distance_points(p1: Rc<RefCell<Point>>, p2: Rc<RefCell<Point>>) -> f32 {
    ((p2.borrow().x - p1.borrow().x).powi(2)
        + (p2.borrow().y - p1.borrow().y).powi(2)
        + (p2.borrow().z - p1.borrow().z).powi(2))
    .sqrt()
}

pub fn calc_distance_point_to_edge(point: Rc<RefCell<Point>>, edge: Rc<RefCell<Edge>>) -> f32 {
    let v1: Vector3<f32> = [
        edge.borrow().p1.borrow().x - point.borrow().x,
        edge.borrow().p1.borrow().y - point.borrow().y,
        edge.borrow().p1.borrow().z - point.borrow().z,
    ];
    let v2: Vector3<f32> = [
        edge.borrow().p1.borrow().x - edge.borrow().p2.borrow().x,
        edge.borrow().p1.borrow().y - edge.borrow().p2.borrow().y,
        edge.borrow().p1.borrow().z - edge.borrow().p2.borrow().z,
    ];
    let f = vec3_len(vec3_cross(v1, v2));
    let s = vec3_len(v2);
    f / s
}

pub fn calc_incircle_radius(
    p1: Rc<RefCell<Point>>,
    p2: Rc<RefCell<Point>>,
    p3: Rc<RefCell<Point>>,
) -> f32 {
    let edge_1_len = calc_distance_points(p1.clone(), p2.clone());
    let edge_2_len = calc_distance_points(p2, p3.clone());
    let edge_3_len = calc_distance_points(p1, p3);

    let s = (edge_1_len + edge_2_len + edge_3_len) / 2.;
    (((s - edge_1_len) * (s - edge_2_len) * (s - edge_3_len)) / s).sqrt()
}

pub fn calc_min_max_angle_of_triangle(e1: Rc<RefCell<Edge>>, e2: Rc<RefCell<Edge>>, e3: Rc<RefCell<Edge>>) -> (f32, f32) {
    let v1 = [
        e1.borrow().p1.borrow().x - e1.borrow().p2.borrow().x,
        e1.borrow().p1.borrow().y - e1.borrow().p2.borrow().y,
        e1.borrow().p1.borrow().z - e1.borrow().p2.borrow().z,
    ];
    let v2 = [
        e2.borrow().p1.borrow().x - e2.borrow().p2.borrow().x,
        e2.borrow().p1.borrow().y - e2.borrow().p2.borrow().y,
        e2.borrow().p1.borrow().z - e2.borrow().p2.borrow().z,
    ];
    let v3 = [
        e3.borrow().p1.borrow().x - e3.borrow().p2.borrow().x,
        e3.borrow().p1.borrow().y - e3.borrow().p2.borrow().y,
        e3.borrow().p1.borrow().z - e3.borrow().p2.borrow().z,
    ];

    let angle1 = (vec3_dot(v1, v2) / (vec3_len(v1) * vec3_len(v2))).acos() * (180. / PI);
    let angle2 = (vec3_dot(v1, v3) / (vec3_len(v1) * vec3_len(v3))).acos() * (180. / PI);
    let angle3 = (vec3_dot(v2, v3) / (vec3_len(v2) * vec3_len(v3))).acos() * (180. / PI);

    let mi = angle1.min(angle2).min(angle3);
    let ma = angle1.max(angle2).max(angle3);
    (mi, ma)
}

pub fn encode_cell(x: isize, y: isize, z: isize) -> isize {
    x | (y << 8) | (z << 16)
}

pub fn decode_cell(code: isize) -> (isize, isize, isize) {
    let mask_x = 0b000000000000000011111111;
    let x = code & mask_x;
    let mask_y = 0b000000001111111100000000;
    let y = (code & mask_y) >> 8;
    let z = code >> 16;
    (x, y, z)
}
