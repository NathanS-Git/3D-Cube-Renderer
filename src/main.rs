use nannou::{App, Frame};
use nannou::geom::pt2;
use nannou::math::map_range;
use nannou::color::{WHITE, BLACK};
use ndarray::prelude::*;

const DIM: usize = 10;

fn main() {
    nannou::sketch(view).run();
}

fn generate_hypercube_vertices() -> Array2<f32> {
    let mut vertices: Array2<f32> = Array2::zeros((DIM, (1 << DIM)));

    for i in 0..DIM { 
        for j in 0..(1 << DIM) {
            vertices[[i,j]] = if (j % (1 << (i+1))) >= (1 << i) { -0.5 } else { 0.5 };
        }
    }

    return vertices;
}

fn construct_rotation_matrix(axis_1: usize, axis_2: usize, angle: f32) -> Array2<f32> {
    let mut rotation_matrix: Array2<f32> = Array2::zeros((DIM, DIM));

    for i in 0..DIM {
        for j in 0..DIM {
            rotation_matrix[[i,j]] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Make the rotation flip every other time
    let angle = if axis_1 % 2 == 0 { angle } else { -angle };
    
    rotation_matrix[[axis_1,axis_2]] = f32::sin(angle);
    rotation_matrix[[axis_2,axis_1]] = -f32::sin(angle);
    rotation_matrix[[axis_1,axis_1]] = f32::cos(angle);
    rotation_matrix[[axis_2,axis_2]] = f32::cos(angle);

    return rotation_matrix; 
}

fn view(app: &App, frame: Frame) {

    let vertices: Array2<f32> = generate_hypercube_vertices();

    // Initialize the rotation matrix to the identity matrix
    let mut rotation_matrix = Array2::zeros((DIM, DIM));
    for i in 0..DIM {
        for j in 0..DIM {
            rotation_matrix[[i,j]] = if i == j { 1.0 } else { 0.0 };
        }
    }
    
    let time = app.time*0.5;
    for i in 0..DIM {
        for j in (i+1)..DIM {
            rotation_matrix = rotation_matrix.dot(&construct_rotation_matrix(i, j, time));
        }
    }

    // Rotate the hypercube
    let vertices = rotation_matrix.dot(&vertices);

    // Draw the hypercube 
    let draw = app.draw();
    draw.background().color(WHITE);

    // Perspective or Orthographic projection
    let perspective_project = true;
    if perspective_project {
        assert!(DIM >= 3, "Perspective projection only works in +3 dimensions");
    }

    let z_depth = if perspective_project { 1.5 } else { 1.0 };
    let zoom = 250.0;

    for i in 0..(1 << DIM) {
        for j in 0..DIM {
            let denominator_1 = if perspective_project { z_depth - vertices[[2, i]] } else { 1.0 };
            let denominator_2 = if perspective_project { z_depth - vertices[[2, i ^ (1 << j)]] } else { 1.0 };

            let x_1 = map_range((vertices[[0, i]]*z_depth) / denominator_1, -1.0, 1.0, -zoom, zoom);
            let x_2 = map_range((vertices[[0, i ^ (1 << j)]]*z_depth) / denominator_2, -1.0, 1.0, -zoom, zoom);

            let y_1 = map_range((vertices[[1, i]]*z_depth) / denominator_1, -1.0, 1.0, -zoom, zoom);
            let y_2 = map_range((vertices[[1, i ^ (1 << j)]]*z_depth) / denominator_2, -1.0, 1.0, -zoom, zoom);
            
            draw.line()
                .start(pt2(x_1, y_1))
                .end(pt2(x_2, y_2))
                .weight(2.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    
}
