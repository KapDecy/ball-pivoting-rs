use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::edge::Edge;
use crate::point::Point;
use crate::utils;

pub struct Grid {
    pub all_points: Rc<RefCell<Vec<Rc<RefCell<Point>>>>>,
    pub cells: HashMap<isize, Vec<Rc<RefCell<Point>>>>,
    pub radius: f32,
    pub num_cells_per_axis: f32,
    pub bounding_box_size: f32,
    pub edges: Vec<Rc<RefCell<Edge>>>,
    pub triangles: Vec<[Rc<RefCell<Point>>; 6]>,
    pub cell_size: f32,
}

impl Grid {
    pub fn new(radius: f32, points: Rc<RefCell<Vec<Rc<RefCell<Point>>>>>) -> Grid {
        let mut grid = Grid {
            all_points: points,
            cells: HashMap::default(),
            radius,
            num_cells_per_axis: 0.0,
            bounding_box_size: 0.0,
            edges: vec![],
            triangles: vec![],
            cell_size: 0.0,
        };
        grid.init_with_data();

        grid
    }

    pub fn init_with_data(&mut self) {
        let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) =
            (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);

        for point in self.all_points.borrow().iter() {
            min_x = min_x.min(point.borrow().x);
            min_y = min_y.min(point.borrow().y);
            min_z = min_z.min(point.borrow().z);
            max_x = max_x.max(point.borrow().x);
            max_y = max_y.max(point.borrow().y);
            max_z = max_z.max(point.borrow().z);
        }

        let x = max_x - min_x;
        let y = max_y - min_y;
        let z = max_z - min_z;

        self.bounding_box_size = x.max(y).max(z);

        self.num_cells_per_axis = self.bounding_box_size / (2. * self.radius);
        self.cell_size = self.bounding_box_size / self.num_cells_per_axis;

        for point in self.all_points.borrow_mut().drain(..) {
            let x_cell = (point.borrow().x / self.cell_size) as isize * self.cell_size as isize;
            let y_cell = (point.borrow().y / self.cell_size) as isize * self.cell_size as isize;
            let z_cell = (point.borrow().z / self.cell_size) as isize * self.cell_size as isize;

            let code = utils::encode_cell(x_cell, y_cell, z_cell);
            point.borrow_mut().cell_code = Some(code);

            if self.cells.contains_key(&code) {
                self.cells.insert(code, vec![]);
            }

            self.cells.get_mut(&code).unwrap().push(point);
        }
    }

    pub fn get_cell_points(&self, cell_code: isize) -> Vec<Rc<RefCell<Point>>> {
        let mut points = vec![];

        if self.cells.contains_key(&cell_code) {
            let p = self.cells.get(&cell_code).unwrap();
            points.extend(p.iter().map(|r| r.to_owned()));
        }

        points
    }

    pub fn add_edge(&mut self, edge: Rc<RefCell<Edge>>) {
        self.edges.push(edge);
    }

    pub fn remove_grid(&mut self, edge: Rc<RefCell<Edge>>) {
        // let e = *edge.borrow();
        let idx = self
            .edges
            .iter()
            .position(|x| *x.borrow() == *edge.borrow())
            .unwrap();
        self.edges.remove(idx);
    }
}
