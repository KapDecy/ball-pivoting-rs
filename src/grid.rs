use std::collections::HashMap;

use crate::edge::Edge;
use crate::point::Point;
use crate::utils::{self, *};

struct Grid<'a> {
    all_points: Vec<Point>,
    cells: HashMap<isize, Vec<Point>>,
    radius: f32,
    num_cells_per_axis: f32,
    bounding_box_size: f32,
    edges: Vec<Edge<'a>>,
    // triangles: Vec<Triangle>,
    cell_size: f32,
}

impl Grid<'_> {
    fn new(radius: f32, points: Option<Vec<Point>>) -> Grid<'static> {
        let all_points = points.unwrap_or(vec![]);
        let mut grid = Grid {
            all_points,
            cells: HashMap::default(),
            radius,
            num_cells_per_axis: 0.0,
            bounding_box_size: 0.0,
            edges: vec![],
            cell_size: 0.0,
        };

        if !grid.all_points.is_empty() {
            grid.init_with_data();
        }

        grid
    }

    fn init_with_data(&mut self) {
        let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) =
            (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);

        for point in &self.all_points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }

        let x = max_x - min_x;
        let y = max_y - min_y;
        let z = max_z - min_z;

        self.bounding_box_size = x.max(y).max(z);

        self.num_cells_per_axis = self.bounding_box_size / (2. * self.radius);
        self.cell_size = self.bounding_box_size / self.num_cells_per_axis;

        for mut point in self.all_points.drain(..) {
            let x_cell = (point.x / self.cell_size) as isize * self.cell_size as isize;
            let y_cell = (point.y / self.cell_size) as isize * self.cell_size as isize;
            let z_cell = (point.z / self.cell_size) as isize * self.cell_size as isize;

            let code = utils::encode_cell(x_cell, y_cell, z_cell);
            point.cell_code = Some(code);

            if self.cells.contains_key(&code) {
                self.cells.insert(code, vec![]);
            }

            self.cells.get_mut(&code).unwrap().push(point);
        }
    }

    fn get_cell_points(&self, cell_code: isize) -> Vec<&Point> {
        let mut points = vec![];

        if self.cells.contains_key(&cell_code) {
            let p = self.cells.get(&cell_code).unwrap();
            points.extend(p.iter());
        }

        points
    }

    fn add_edge(&mut self, edge: Edge<'static>) {
        self.edges.push(edge);
    }

}
