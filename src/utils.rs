use std::f32::consts::PI;

use crate::edge::Edge;
use crate::point::Point;
use vecmath::{vec3_cross, vec3_dot, vec3_len, Vector3};

pub fn calc_distance_points(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt()
}

pub fn calc_distance_point_to_edge(point: &Point, edge: &Edge) -> f32 {
    let v1: Vector3<f32> = [
        edge.p1.x - point.x,
        edge.p1.y - point.y,
        edge.p1.z - point.z,
    ];
    let v2: Vector3<f32> = [
        edge.p1.x - edge.p2.x,
        edge.p1.y - edge.p2.y,
        edge.p1.z - edge.p2.z,
    ];
    let f = vec3_len(vec3_cross(v1, v2));
    let s = vec3_len(v2);
    f / s
}

pub fn calc_incircle_radius(p1: &Point, p2: &Point, p3: &Point) -> f32 {
    let edge_1_len = calc_distance_points(p1, p2);
    let edge_2_len = calc_distance_points(p2, p3);
    let edge_3_len = calc_distance_points(p1, p3);

    let s = (edge_1_len + edge_2_len + edge_3_len) / 2.;
    (((s - edge_1_len) * (s - edge_2_len) * (s - edge_3_len)) / s).sqrt()
}

pub fn calc_min_max_angle_of_triangle(e1: &Edge, e2: &Edge, e3: &Edge) -> (f32, f32) {
    let v1 = [e1.p1.x - e1.p2.x, e1.p1.y - e1.p2.y, e1.p1.z - e1.p2.z];
    let v2 = [e2.p1.x - e2.p2.x, e2.p1.y - e2.p2.y, e2.p1.z - e2.p2.z];
    let v3 = [e3.p1.x - e3.p2.x, e3.p1.y - e3.p2.y, e3.p1.z - e3.p2.z];

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
