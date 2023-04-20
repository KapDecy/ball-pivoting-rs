use ball_pivoting_rs::{point::Point, edge::Edge, utils::calc_distance_point_to_edge};



fn main() {
    let p = Point::new(0., 0., 0., 0);
    let p1 = Point::new(-3., 5., 0., 0);
    let p2 = Point::new(3., 5., 0., 0);
    let edge = Edge::new(p1, p2);

    println!("{}", calc_distance_point_to_edge(p, edge));
}
