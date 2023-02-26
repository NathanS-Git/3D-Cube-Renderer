use nannou::winit::platform::unix::x11::ffi::Depth;
use nannou::{App, Frame};
use nannou::geom::pt2;
use nannou::math::map_range;
use nannou::color::{WHITE, BLACK};
use ndarray::prelude::*;

const DIM: usize = 4;

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

    // Make rotations occur in different directions
    if axis_2 % 2 == 0 {
        rotation_matrix[[axis_1,axis_2]] = f32::sin(angle);
        rotation_matrix[[axis_2,axis_1]] = -f32::sin(angle);
    } else {
        rotation_matrix[[axis_1,axis_2]] = -f32::sin(angle);
        rotation_matrix[[axis_2,axis_1]] = f32::sin(angle);
    }

    rotation_matrix[[axis_1,axis_1]] = f32::cos(angle);
    rotation_matrix[[axis_2,axis_2]] = f32::cos(angle);

    return rotation_matrix; 
}

fn view(app: &App, frame: Frame) {

    let vertices: Array2<f32> = generate_hypercube_vertices();
    
    let mut rotation_matrix = Array2::zeros((DIM, DIM));

    for i in 0..DIM {
        for j in 0..DIM {
            rotation_matrix[[i,j]] = if i == j { 1.0 } else { 0.0 };
        }
    }
    
    let time = app.time*0.3;
    for i in 0..DIM {
        for j in (i+1)..DIM {
            rotation_matrix = rotation_matrix.dot(&construct_rotation_matrix(i, j, time));
        }
    }

    let vertices = rotation_matrix.dot(&vertices);
    let draw = app.draw();

    draw.background().color(WHITE);

    let vertices = rotation_matrix.dot(&vertices);

    let z_depth = 1.5;

    for i in 0..(1 << DIM) {
        for j in 0..DIM {
            // Perspective project the vertex
            // not Orthographically project the vertex

            let x_1 = map_range((vertices[[0, i]]*z_depth) / (z_depth - vertices[[2, i]]), -1.0, 1.0, -100.0, 100.0);
            let x_2 = map_range((vertices[[0, i ^ (1 << j)]]*z_depth) / (z_depth - vertices[[2, i ^(1 << j)]]), -1.0, 1.0, -100.0, 100.0);

            let y_1 = map_range((vertices[[1, i]]*z_depth) / (z_depth - vertices[[2, i]]), -1.0, 1.0, -100.0, 100.0);
            let y_2 = map_range((vertices[[1, i ^ (1 << j)]]*z_depth) / (z_depth - vertices[[2, i ^(1 << j)]]), -1.0, 1.0, -100.0, 100.0);
            draw.line()
                .start(pt2(x_1, y_1))
                .end(pt2(x_2, y_2))
                .weight(2.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    
}