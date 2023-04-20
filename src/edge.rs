use crate::point::Point;


#[derive(Clone)]
pub struct Edge<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
    pub num_triangles_this_edge_in: usize,
}

impl Edge<'_> {
    pub fn new<'a>(p1: &'a Point, p2: &'a Point) -> Edge<'a> {
        Edge {
            p1,
            p2,
            num_triangles_this_edge_in: 0,
        }
    }
}