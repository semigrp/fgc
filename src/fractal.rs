use delaunay_creator::{Point2D, Triangle, bowyer_watson};
use rand::Rng;

pub struct Fractal {
    depth: usize,
    size: usize,
    triangles: Vec<Triangle>,
}

impl Fractal {
    pub fn new(depth: usize, size: usize, num_points: usize) -> Self {
        let points = Fractal::generate_random_points(size, num_points);
        let triangles = bowyer_watson(points);
        Fractal { depth, size, triangles }
    }

    fn generate_random_points(size: usize, num_points: usize) -> Vec<Point2D> {
        let mut rng = rand::thread_rng();
        (0..num_points)
            .map(|_| Point2D {
                x: rng.gen_range(0.0..size as f64),
                y: rng.gen_range(0.0..size as f64),
            })
            .collect()
    }

    fn generate_fractal(&self, triangle: &Triangle, depth: usize) -> Vec<Triangle> {
        if depth == 0 {
            vec![*triangle]
        } else {
            let a = triangle.a;
            let b = triangle.b;
            let c = triangle.c;
            let m1 = Point2D {
                x: (a.x + b.x) / 2.0,
                y: (a.y + b.y) / 2.0,
            };
            let m2 = Point2D {
                x: (b.x + c.x) / 2.0,
                y: (b.y + c.y) / 2.0,
            };
            let m3 = Point2D {
                x: (c.x + a.x) / 2.0,
                y: (c.y + a.y) / 2.0,
            };
            let t1 = Triangle { a, b: m1, c: m3 };
            let t2 = Triangle { a: m1, b, c: m2 };
            let t3 = Triangle { a: m3, b: m2, c };

            [
                self.generate_fractal(&t1, depth - 1),
                self.generate_fractal(&t2, depth - 1),
                self.generate_fractal(&t3, depth - 1),
                ]
                .into_iter()
                .flatten()
                .collect()
            }
        }
    
        pub fn generate(&self) -> Vec<Triangle> {
            self.triangles
                .iter()
                .flat_map(|triangle| self.generate_fractal(triangle, self.depth))
                .collect()
        }
    }
