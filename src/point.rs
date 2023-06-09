use std::{cell::RefCell, rc::Rc};
use vecmath::Vector3;

use crate::utils::{self, *};

#[derive(PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cell_code: Option<isize>,
    pub normal: Option<Vector3<f32>>,
    pub id: usize,
    pub is_used: bool,
}

impl Point {
    // TODO cell_node and normal
    pub fn new(x: f32, y: f32, z: f32, id: usize, normal: Option<Vector3<f32>>) -> Rc<RefCell<Point>> {
        Rc::new(RefCell::new(Point {
            x,
            y,
            z,
            cell_code: None,
            normal,
            id,
            is_used: false,
        }))
    }

    pub fn neighbor_nodes(&self) -> Vec<isize> {
        let mut neighbor_nodes = vec![self.cell_code.unwrap()];

        let (x, y, z) = decode_cell(self.cell_code.unwrap());

        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    let cell_corner = (x + i, y + j, z + k);

                    if cell_corner.0 < 0 || cell_corner.1 < 0 || cell_corner.2 < 0 {
                        continue;
                    }

                    let cell_code = utils::encode_cell(cell_corner.0, cell_corner.1, cell_corner.2);
                    neighbor_nodes.push(cell_code);
                }
            }
        }

        neighbor_nodes
    }
}
