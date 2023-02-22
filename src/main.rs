use nannou::prelude::{App, Frame, Draw, WHITE, BLACK, pt2, map_range};
use ndarray::{arr0, Array2, Axis, s};

fn main() {
    nannou::sketch(view).run();
}

fn generate_hypercube_vertices() -> Array2<f32> {
    const DIM: usize = 3;

    let mut vertices: Array2<f32> = Array2::zeros(((1 << DIM), DIM));

    // Generate the vertices of the hypercube
    for i in 0..DIM { 
        for j in 0..(1 << DIM) {
            vertices[[j,i]] = if (j % (1 << (i+1))) >= (1 << i) { -0.5 } else { 0.5 };
        }
    }

    // Generate the edges of the hypercube
    /*
    Figuring out a method to find the edges given a vertex index...

    0: 0 0 0
    1: 1 0 0
    2: 0 1 0
    3: 1 1 0
    4: 0 0 1
    5: 1 0 1
    6: 0 1 1 
    7: 1 1 1

    0 -> (1, 2, 4)
    1 -> (0, 3, 5)
    2 -> (0, 3, 6)
    3 -> (1, 2, 7)
    4 -> (0, 5, 6)
    5 -> (1, 4, 7)
    6 -> (2, 4, 7)
    7 -> (3, 5, 6)


    1: 0 0 0 0
    2: 1 0 0 0
    3: 0 1 0 0
    4: 1 1 0 0
    5: 0 0 1 0
    6: 1 0 1 0
    7: 0 1 1 0
    8: 1 1 1 0
    9: 0 0 0 1
    10: 1 0 0 1
    11: 0 1 0 1
    12: 1 1 0 1
    13: 0 0 1 1
    14: 1 0 1 1
    15: 0 1 1 1
    16: 1 1 1 1


    1 -> (2, 3, 5, 9)
    2 -> (1, 4, 6, 10)
    3 -> (1, 4, 7, 11)
    4 -> (2, 3, 8, 12)
    5 -> (1, 6, 7, 13)
    6 -> (2, 5, 8, 14)
    7 -> (3, 5, 8, 15)
    8 -> (4, 6, 7, 16)
    9 -> (1, 10, 11, 13)
    10 -> (2, 9, 12, 14)
    11 -> (3, 9, 12, 15)
    12 -> (4, 10, 11, 16)
    13 -> (5, 9, 14, 15)
    14 -> (6, 10, 13, 16)
    15 -> (7, 11, 13, 16)
    16 -> (8, 12, 14, 15)

    1 -> (2, 3, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385, 32769)
    2 -> (1, 4, 6, 10, 18, 34, 66, 130, 258, 514, 1026, 2050, 4098, 8194, 16386, 32770)
    3 -> (1, 4, 7, 11, 19, 35, 67, 131, 259, 515, 1027, 2051, 4099, 8195, 16387, 32771)
    4 -> (2, 3, 8, 12, 20, 36, 68, 132, 260, 516, 1028, 2052, 4100, 8196, 16388, 32772)
    5 -> (1, 6, 7, 13, 21, 37, 69, 133, 261, 517, 1029, 2053, 4101, 8197, 16389, 32773)
    6 -> (2, 5, 8, 14, 22, 38, 70, 134, 262, 518, 1030, 2054, 4102, 8198, 16390, 32774)
    7 -> (3, 5, 8, 15, 23, 39, 71, 135, 263, 519, 1031, 2055, 4103, 8199, 16391, 32775)
    8 -> (4, 6, 7, 16, 24, 40, 72, 136, 264, 520, 1032, 2056, 4104, 8200, 16392, 32776)
    9 -> (1, 10, 11, 13, 25, 41, 73, 137, 265, 521, 1033, 2057, 4105, 8201, 16393, 32777)
    10 -> (2, 9, 12, 14, 26, 42, 74, 138, 266, 522, 1034, 2058, 4106, 8202, 16394, 32778)
    
    */    


    for i in 0..(1 << DIM) {
        println!("{}: {},{},{}", i, i ^ 1, i ^ (1 << 1), i ^ (1 << 2));
    }

    println!("{:?}", vertices);
    return vertices;
}

fn construct_rotation_matrix() -> Array2<f32> {
    const DIM: usize = 3;

    let mut rotation_matrix: Array2<f32> = Array2::zeros((DIM, DIM));

    for i in 0..DIM {
        for j in 0..DIM {
            rotation_matrix[[i,j]] = if i == j { 1.0 } else { 0.0 };
        }
    }

    println!("{:?}", rotation_matrix);
    return rotation_matrix;
}

fn view(app: &App, frame: Frame) {


    let vertices: Array2<f32> = generate_hypercube_vertices();
    let rotation_matrix: Array2<f32> = construct_rotation_matrix();

    let draw = app.draw();

    // Build up the rotation matrix
    
    draw.background().color(WHITE);

    //let sine = app.time.sin();

    // Draw the wire-frame of the hypercube
    

    let boundary = app.window_rect();

    const DIM: usize = 3;
    for i in 0..(1 << DIM) {
        for j in 0..DIM {
            let x_1 = map_range(vertices[[i,0]], -1.0, 1.0, boundary.left(), boundary.right());
            let x_2 = map_range(vertices[[i ^ (1 << j),0]], -1.0, 1.0, boundary.left(), boundary.right());
            let y_1 = map_range(vertices[[i,1]], -1.0, 1.0, boundary.bottom(), boundary.top());
            let y_2 = map_range(vertices[[i ^ (1 << j),1]], -1.0, 1.0, boundary.bottom(), boundary.top());
            draw.line()
                .start(pt2(x_1, y_1))
                .end(pt2(x_2, y_2))
                .weight(2.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    
}