use std::{cell::RefCell, collections::HashSet, path::PathBuf, rc::Rc};
use std::cmp::Ordering::Equal;
use std::f32::consts::E;
use std::ops::Deref;

use itertools::Itertools;
use vecmath::{vec3_cross, vec3_dot};

use crate::{edge::Edge, grid::Grid, point::Point, utils};
use crate::utils::{calc_distance_points, calc_incircle_radius, calc_min_max_angle_of_triangle};

struct BPA {
    first_free_point_index: usize,
    num_points_i_tried_to_seem_from: usize,
    points: Rc<RefCell<Vec<Rc<RefCell<Point>>>>>,
    radius: f32,
    grid: Grid,
    num_free_points: usize,
    num_workers: usize,
}

impl BPA {
    pub fn new(points: Vec<Rc<RefCell<Point>>>, radius: f32, num_workers: usize) -> BPA {
        let rcpoints = Rc::new(RefCell::new(points));
        let rcpointslen = rcpoints.borrow().len();
        BPA {
            first_free_point_index: 0,
            num_points_i_tried_to_seem_from: 0,
            points: rcpoints.clone(),
            radius,
            grid: Grid::new(radius, rcpoints.clone()),
            num_free_points: rcpointslen,
            num_workers,
        }
    }

    pub fn get_points_distances_from_edge(
        points: Vec<Rc<RefCell<Point>>>,
        p1: Rc<RefCell<Point>>,
        p2: Rc<RefCell<Point>>,
    ) -> Vec<f32> {
        // TODO add round to 2 digits
        let dists_p1 = points
            .iter()
            .map(|p3| utils::calc_distance_points(p1.clone(), p3.clone()))
            .collect::<Vec<_>>();
        let dists_p2 = points
            .iter()
            .map(|p3| utils::calc_distance_points(p2.clone(), p3.clone()))
            .collect::<Vec<_>>();
        let dists = dists_p1
            .iter()
            .zip(dists_p2)
            .map(|(&a, b)| a + b)
            .collect_vec();
        dists
    }

    pub fn get_third_point_of_triangle(
        triangle_edges: Vec<Rc<RefCell<Edge>>>,
        p1: Rc<RefCell<Point>>,
        p2: Rc<RefCell<Point>>,
    ) -> Option<Rc<RefCell<Point>>> {
        let mut triangle_points = vec![];

        for triangel_edge in triangle_edges.iter() {
            // let first_point = triangel_edge.borrow().p1;
            // let second_point = triangel_edge.borrow().p2;
            triangle_points.push(triangel_edge.borrow().p1.clone());
            triangle_points.push(triangel_edge.borrow().p2.clone());
        }

        let set = triangle_points
            .iter()
            .unique_by(|p| p.borrow().id)
            .cloned()
            .collect_vec();

        let mut rp = None;

        for point in set.iter() {
            if point.borrow().id != p1.borrow().id && point.borrow().id != p2.borrow().id {
                rp = Some(point.clone());
            }
        }

        rp
    }

    pub fn will_triangles_overlap(
        edge: Rc<RefCell<Edge>>,
        p3: Rc<RefCell<Point>>,
        p4: Rc<RefCell<Point>>,
    ) -> bool {
        let (p1, p2) = (edge.borrow().p1.clone(), edge.borrow().p2.clone());

        let v1 = [
            p3.borrow().x - p1.borrow().x,
            p3.borrow().y - p1.borrow().y,
            p3.borrow().z - p1.borrow().z,
        ];
        let v2 = [
            p2.borrow().x - p1.borrow().x,
            p2.borrow().y - p1.borrow().y,
            p2.borrow().z - p1.borrow().z,
        ];

        let triangle_normal = vec3_cross(v1, v2);

        let plane_normal = vec3_cross(v2, triangle_normal);

        let v3 = [
            p4.borrow().x - p1.borrow().x,
            p4.borrow().y - p1.borrow().y,
            p4.borrow().z - p1.borrow().z,
        ];

        vec3_dot(plane_normal, v1).signum() == vec3_dot(plane_normal, v3).signum()
    }

    pub fn create_mesh(&mut self, limit_iterations: Option<usize>, first_point_index: usize) {
        let mut tried_to_expand_counter = 0;

        todo!()
    }

    pub fn find_seed_triangle(&mut self, mut first_point_index: usize, num_recursion_calls: usize) -> (isize, (Rc<RefCell<Edge>>, Rc<RefCell<Edge>>, Rc<RefCell<Edge>>), usize) {
        if num_recursion_calls > self.points.borrow().len() {
            println!("i was here");
            panic!("strange shit in \"find_seed_triangle\"")
            // return (-1, vec![-1, -1]);
        }

        if first_point_index >= (self.points.borrow().len() - 1) {
            first_point_index = 0;
        }

        let p1 = self.points.borrow()[first_point_index].clone();
        let mut p1_neighbor_points = vec![];

        for cell in p1.borrow().neighbor_nodes() {
            p1_neighbor_points.extend(self.grid.get_cell_points(cell));
        }

        let p1_neighbor_points = p1_neighbor_points.iter().unique_by(|&p| p.borrow().id)
            .map(|p| p.clone()).collect_vec();

        let dists = p1_neighbor_points.iter().map(|p2| calc_distance_points(p1.clone(), p2.clone())).collect_vec();
        let p1_neighbor_points = dists.iter().zip(p1_neighbor_points).
            sorted_by(|(d1, p1), (d2,p2)| d1.partial_cmp(d2).unwrap_or(Equal))
            .map(|(d, p)| p.clone()).collect_vec();

        let limit_points = 6;
        let p1_neighbor_points = p1_neighbor_points[..=limit_points].iter().collect_vec();

        for p2 in p1_neighbor_points {
            // if p2.borrow().is_used {
            //
            // }

            if p2.borrow().x == p1.borrow().x && p2.borrow().y == p1.borrow().y && p2.borrow().z == p1.borrow().z {
                continue
            }

            let mut intersect_cells = p1.borrow().neighbor_nodes();
            intersect_cells.extend(p2.borrow().neighbor_nodes());
            let intersect_cells = intersect_cells.iter().unique().collect_vec();

            let mut possible_points = vec![];

            for cell in intersect_cells{
                possible_points.extend(self.grid.get_cell_points(*cell));
            }

            let dists_p2 = possible_points.iter().map(|p3| calc_distance_points(p2.clone(), p3.clone())).collect_vec();
            let dists_p1 = possible_points.iter().map(|p3| calc_distance_points(p1.clone(), p3.clone())).collect_vec();

            let dists = (0..dists_p1.len()).into_iter().map(|i| dists_p1[i] + dists_p2[i]).collect_vec();
            let possible_points = dists.iter().zip(possible_points).
                sorted_by(|(d1, p1), (d2,p2)| d1.partial_cmp(d2).unwrap_or(Equal))
                .map(|(d, p)| p.clone()).collect_vec();

            let limit_points = 5;
            let possible_points = possible_points[..limit_points].iter().collect_vec();

            for p3 in possible_points.iter().cloned() {
                if (p3.borrow().x == p1.borrow().x && p3.borrow().y == p1.borrow().y && p3.borrow().z == p1.borrow().z)
                    || (p2.borrow().x == p3.borrow().x && p2.borrow().y == p3.borrow().y && p2.borrow().z == p3.borrow().z) {
                    continue;
                }

                if self.radius <= calc_incircle_radius(p1.clone(), p2.clone(), p3.clone()) {
                    let v1 = [p2.borrow().x - p1.borrow().x, p2.borrow().y - p1.borrow().y, p2.borrow().z- p1.borrow().z];
                    let v2 = [p3.borrow().x - p1.borrow().x, p3.borrow().y - p1.borrow().y, p3.borrow().z- p1.borrow().z];

                    let triangle_normal = vec3_cross(v1, v2);

                    //TODO: Check if the normal of the triangle is on the same direction with points normals.
                    //https://github.com/Lotemn102/Ball-Pivoting-Algorithm/blob/34edabac94a4ecbc01741ecc76df0d24cd6f1e2c/bpa.py#L262


                    // p1_and_p3_already_connected = [e for e in self.grid.edges if ((e.p1.id == p1.id)
                    // and (e.p2.id == p3.id)) or ((e.p1.id == p3.id) and (e.p2.id == p1.id))]
                    let p1_and_p3_already_connected = self.grid.edges.iter().filter(|&e| {
                        let eb = e.borrow();
                        ((eb.p1.borrow().id == p1.borrow().id) && (eb.p2.borrow().id == p3.borrow().id)) ||
                            ((eb.p1.borrow().id == p3.borrow().id) && (eb.p2.borrow().id == p1.borrow().id))
                    }).collect_vec();

                    let p1_and_p2_already_connected = self.grid.edges.iter().filter(|&e| {
                        let eb = e.borrow();
                        ((eb.p1.borrow().id == p1.borrow().id) && (eb.p2.borrow().id == p2.borrow().id)) ||
                            ((eb.p1.borrow().id == p2.borrow().id) && (eb.p2.borrow().id == p1.borrow().id))
                    }).collect_vec();

                    let p2_and_p3_already_connected = self.grid.edges.iter().filter(|&e| {
                        let eb = e.borrow();
                        ((eb.p1.borrow().id == p2.borrow().id) && (eb.p2.borrow().id == p3.borrow().id)) ||
                            ((eb.p1.borrow().id == p3.borrow().id) && (eb.p2.borrow().id == p2.borrow().id))
                    }).collect_vec();

                    if p1_and_p3_already_connected.len() > 0 || p1_and_p2_already_connected.len() > 0 || p2_and_p3_already_connected.len() >0 {
                        continue;
                    }

                    let are_p1_p3_closing_another_triangle_in_the_mesh =
                        self.is_there_a_path_between_two_points(p1.clone(), p3.clone(), p2.clone());
                    let are_p2_p3_closing_another_triangle_in_the_mesh =
                        self.is_there_a_path_between_two_points(p2.clone(), p3.clone(), p1.clone());
                    let are_p1_p2_closing_another_triangle_in_the_mesh =
                        self.is_there_a_path_between_two_points(p1.clone(), p2.clone(), p3.clone());

                    let mut e1 = Edge::new(p1.clone(), p3.clone());
                    e1.borrow_mut().num_triangles_this_edge_in += 1;
                    if are_p1_p3_closing_another_triangle_in_the_mesh {
                        e1.borrow_mut().num_triangles_this_edge_in += 1;
                    }
                    let mut e2 = Edge::new(p1.clone(), p2.clone());
                    e2.borrow_mut().num_triangles_this_edge_in += 1;
                    if are_p1_p2_closing_another_triangle_in_the_mesh {
                        e2.borrow_mut().num_triangles_this_edge_in += 1;
                    }
                    let mut e3 = Edge::new(p2.clone(), p3.clone());
                    e3.borrow_mut().num_triangles_this_edge_in += 1;
                    if are_p2_p3_closing_another_triangle_in_the_mesh {
                        e3.borrow_mut().num_triangles_this_edge_in += 1;
                    }

                    let (min_angle, max_angle) = calc_min_max_angle_of_triangle(e1.clone(), e2.clone(), e3.clone());

                    if max_angle > 170. || min_angle < 20. {
                        continue
                    }

                    self.grid.edges.push(e1.clone());
                    self.grid.edges.push(e2.clone());
                    self.grid.edges.push(e3.clone());

                    let mut triangle =
                        [e1.borrow().p1.clone(), e1.borrow().p2.clone(), e2.borrow().p1.clone(), e2.borrow().p2.clone(),e3.borrow().p1.clone(), e3.borrow().p2.clone()];
                    triangle.sort_by(|p1, p2| {p1.borrow().z.total_cmp(&p2.borrow().z)});

                    self.grid.triangles.push(triangle);
                    self.first_free_point_index += 1;

                    p1.borrow_mut().is_used = true;
                    p2.borrow_mut().is_used = true;
                    p2.borrow_mut().is_used = true;

                    return (1, (e1, e2, e3), first_point_index);
                }
            }
        }

        self.find_seed_triangle(first_point_index + 1, num_recursion_calls +1)
    }

    fn is_there_a_path_between_two_points(&self, p1: Rc<RefCell<Point>>, p2: Rc<RefCell<Point>>, point_of_triangle_we_creating: Rc<RefCell<Point>>) -> bool {
        let mut edges_first_point_int = vec![];
        let mut edges_second_point_int = vec![];
        let mut points_first_edges = vec![];
        let mut points_second_edges = vec![];

        for e in self.grid.edges.iter() {
            if p1.borrow().id == e.borrow().p1.borrow().id || p1.borrow().id == e.borrow().p2.borrow().id {
                edges_first_point_int.push(e.clone());
            }

            if p2.borrow().id == e.borrow().p1.borrow().id || p2.borrow().id == e.borrow().p2.borrow().id {
                edges_second_point_int.push(e.clone());
            }
        }

        for e in edges_first_point_int.iter() {
            points_first_edges.push(e.borrow().p1.borrow().id);
            points_first_edges.push(e.borrow().p2.borrow().id);
        }
        for e in edges_second_point_int.iter() {
            points_second_edges.push(e.borrow().p1.borrow().id);
            points_second_edges.push(e.borrow().p2.borrow().id);
        }

        let mut points_first_edges = points_first_edges.iter().unique_by(|&p| p).map(|p| p.clone()).collect_vec();
        if points_first_edges.contains(&(p1.borrow().id)) {
            points_first_edges.remove(points_first_edges.iter().position(|x| *x == p1.borrow().id).unwrap());
        }

        let mut points_second_edges = points_second_edges.iter().unique_by(|&p| p).map(|p| p.clone()).collect_vec();
        if points_second_edges.contains(&(p1.borrow().id)) {
            points_second_edges.remove(points_second_edges.iter().position(|x| *x == p1.borrow().id).unwrap());
        }

        points_first_edges.extend(points_second_edges);
        let mut intersection = points_first_edges.iter().unique().map(|id| *id).collect_vec();

        if intersection.contains(&point_of_triangle_we_creating.borrow().id) {
            intersection.remove(intersection.iter().position(|x| *x == point_of_triangle_we_creating.borrow().id).unwrap());
        }

        intersection.len() > 0
    }
}
