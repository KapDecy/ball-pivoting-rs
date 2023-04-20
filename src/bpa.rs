use std::{cell::RefCell, collections::HashSet, path::PathBuf, rc::Rc};

use itertools::Itertools;

use crate::{edge::Edge, grid::Grid, point::Point, utils};

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
}
