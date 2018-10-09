#[derive(Debug, Clone)]
pub struct Quadrant {
    center_of_mass: Option<(f64, f64)>,
    domain_range: (f64, f64, f64, f64),
    local_body: Option<Body>,
    child_quadrants: (
        Option<Box<Quadrant>>,
        Option<Box<Quadrant>>,
        Option<Box<Quadrant>>,
        Option<Box<Quadrant>>,
    ),
}

impl Quadrant {
    // domain_range := (x-min, x-max, y-min, y-max)
    pub fn new(domain_range: (f64, f64, f64, f64)) -> Quadrant {
        Quadrant {
            center_of_mass: None,
            domain_range: domain_range,
            local_body: None,
            child_quadrants: (None, None, None, None),
        }
    }

    pub fn create_child_at_index(&mut self, index: i64, domain_range: (f64, f64, f64, f64)) {
        let new_quad = Quadrant::new(domain_range);
        match index {
            0 => self.child_quadrants.0 = Some(Box::new(new_quad)),
            1 => self.child_quadrants.1 = Some(Box::new(new_quad)),
            2 => self.child_quadrants.2 = Some(Box::new(new_quad)),
            3 => self.child_quadrants.3 = Some(Box::new(new_quad)),
            _ => ()
        }
    }

    pub fn create_child_quadrants(&mut self, domains_ranges: [(f64, f64, f64, f64); 4]) {
        match self.child_quadrants {
            (None, None, None, None) => {
                self.create_child_at_index(0, domains_ranges[0]);
                self.create_child_at_index(1, domains_ranges[1]);
                self.create_child_at_index(2, domains_ranges[2]);
                self.create_child_at_index(3, domains_ranges[3]);
            },
            (_, _, _, _) => ()
        }
    }
}

pub fn split_parent_domain_into_quarters(domain: (f64, f64, f64, f64)) -> [(f64, f64, f64, f64); 4] {
    /*
        [ (q1_x_min,q1_x_max q1_y_min, q1_y_max) ], etc

        Quadrants are named after the standard 2D Cartesian Plane, i.e.
         q2  |   q1
        _____|_____
             |
         q3  |   q4
    */
    let x_min: f64 = domain.0;
    let x_max: f64 = domain.1;
    let y_min: f64 = domain.2;
    let y_max: f64 = domain.3;

    let q1 = ((x_max - x_min)/2.0, x_max, y_min, (y_max - y_min)/2.0);
    let q2 = (x_min, ((x_max - x_min)/2.0), y_min, (y_max - y_min)/2.0);
    let q3 = (x_min, ((x_max - x_min)/2.0), (y_max - y_min)/2.0, y_max);
    let q4 = ((x_max - x_min)/2.0, x_max, (y_max - y_min)/2.0, y_max);

    [q1, q2, q3, q4]
}

pub fn build_quadrant_space(_bodies: &[Body]) {
    let mut mother_quadrant: &mut Quadrant = &mut Quadrant::new((0.0, 500.0, 0.0, 500.0));
    for body in _bodies.iter() {
        insert_into_quadrant(*body, &mut mother_quadrant);
    }
    println!("{:?}", mother_quadrant);
}

pub fn find_quadrant_index_to_place_body(body_coords: (f64, f64), quadrant_dimensions: &[(f64, f64, f64, f64); 4]) -> usize {
    let body_x = body_coords.0;
    let body_y = body_coords.1;

    for (index, quad_dim) in quadrant_dimensions.iter().enumerate() {
        if (quad_dim.0 <= body_x && quad_dim.1 > body_x) && (quad_dim.2 <= body_y && quad_dim.3 > body_y) {
            return index;
        }
    }
    return 0;
}

pub fn insert_into_quadrant(body: Body, quadrant: &mut Quadrant) {
    println!("BODY: {:?}", body);
    match quadrant.local_body {
        Some(body) => {
            let children_domains_ranges = split_parent_domain_into_quarters(quadrant.domain_range);
            quadrant.create_child_quadrants(children_domains_ranges);

            // Find which quadrant the body _should_ go into and pass that into the next call
            let index: usize = find_quadrant_index_to_place_body(body.coords, &children_domains_ranges);
            println!("Index {}", index);
            match index {
                0 => insert_into_quadrant(body, &mut *quadrant.child_quadrants.0.as_mut().unwrap()),
                1 => insert_into_quadrant(body, &mut *quadrant.child_quadrants.1.as_mut().unwrap()),
                2 => insert_into_quadrant(body, &mut *quadrant.child_quadrants.2.as_mut().unwrap()),
                3 => insert_into_quadrant(body, &mut *quadrant.child_quadrants.3.as_mut().unwrap()),
                _ => (),
            }
        },
        None => {
            quadrant.local_body = Some(body);
        }
    }

}

#[derive(Clone, Copy, Debug)]
pub struct Body {
    coords: (f64, f64),
    mass: f64,
}

// Implementation example
/*
fn main() {
    let b1: Body = Body { coords: (150.0, 100.0), mass: 6.0 };
    let b2: Body = Body { coords: (260.0, 100.0), mass: 6.0 };
    let b3: Body = Body { coords: (350.0, 200.0), mass: 6.0 };
    let b4: Body = Body { coords: (150.0, 70.0), mass: 6.0 };
    let b5: Body = Body { coords: (155.0, 170.0), mass: 6.0 };
    let _bodies = [b1, b2, b3, b4, b5];
    build_quadrant_space(&_bodies);

    //let mut node: Quadrant = Quadrant::new();
    //node.create_child_quadrants();
    //println!("Node: {:?}", &node);
}
*/
