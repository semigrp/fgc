use std::io::Write;
use delaunay_creator::{Point2D, Triangle};

use crate::fractal::Fractal;

pub fn draw_fractal(fractal: &Fractal, size: usize) {
    let triangles = fractal.generate();
    let mut stdout = std::io::stdout();

    for y in 0..size {
        for x in 0..size {
            let point = Point2D {
                x: x as f64,
                y: y as f64,
            };
            let mut display_char = ' ';

            for triangle in &triangles {
                if triangle_contains_point(triangle, &point) {
                    display_char = '*';
                    break;
                }
            }

            if display_char == '*' {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    stdout.flush().unwrap();
}



pub fn triangle_contains_point(triangle: &Triangle, point: &Point2D) -> bool {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    let v0 = Point2D {
        x: c.x - a.x,
        y: c.y - a.y,
    };
    let v1 = Point2D {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let v2 = Point2D {
        x: point.x - a.x,
        y: point.y - a.y,
    };

    let dot00 = v0.x * v0.x + v0.y * v0.y;
    let dot01 = v0.x * v1.x + v0.y * v1.y;
    let dot02 = v0.x * v2.x + v0.y * v2.y;
    let dot11 = v1.x * v1.x + v1.y * v1.y;
    let dot12 = v1.x * v2.x + v1.y * v2.y;

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u > 0.0) && (v > 0.0) && (u + v < 1.0)
}