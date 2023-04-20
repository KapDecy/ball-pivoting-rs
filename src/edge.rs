use std::{cell::RefCell, rc::Rc};

use crate::point::Point;

#[derive(Clone, PartialEq)]
pub struct Edge {
    pub p1: Rc<RefCell<Point>>,
    pub p2: Rc<RefCell<Point>>,
    pub num_triangles_this_edge_in: usize,
}

impl Edge {
    pub fn new(p1: Rc<RefCell<Point>>, p2: Rc<RefCell<Point>>) -> Rc<RefCell<Edge>> {
        Rc::new(RefCell::new(Edge {
            p1,
            p2,
            num_triangles_this_edge_in: 0,
        }))
    }
}
