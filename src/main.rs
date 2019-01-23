mod matrix;
use matrix::Matrix;

fn to_echelon_form(matrix: &mut Matrix<f64>) {
    assert!(matrix.rows() <= matrix.cols());

    let mut row = 0;
    for c in 0..matrix.cols() {
        if row >= matrix.rows() {
            return
        }
        for r in row..matrix.rows() {
            if matrix.row(r)[c] != 0.0 {
                break;
            }
            matrix.swap_rows(row, r);
        }
        if matrix.row(row)[c] == 0.0 {
            continue
        }
        matrix.multiply_row(row, &(-1.0 / matrix.row(row)[c]));
        for r in row+1..matrix.rows() {
            if matrix.row(r)[c] != 0.0 {
                matrix.multiply_row(r, &(1.0 / matrix.row(r)[c]));
                matrix.add_row(r, row);
            }
        }
        matrix.multiply_row(row, &(-1.0));
        row += 1;
    }
}

fn to_reduced_echelon_form(matrix: &mut Matrix<f64>) {
    assert!(matrix.rows() <= matrix.cols());

    let mut row = 0;
    for c in 0..matrix.cols() {
        if row >= matrix.rows() {
            return
        }
        if matrix.row(row)[c] == 0.0 {
            continue
        }
        
        for r in 0..row {
            let f = -matrix.row(r)[c];
            matrix.add_row_multiple(r, row, &f);
        }

        row += 1;
    }
}

fn solve_system_of_linear_equations(mut system: Matrix<f64>) -> Option<Vec<f64>> {
    to_echelon_form(&mut system);
    to_reduced_echelon_form(&mut system);

    let mut solution = Vec::new();
    let mut c = 0;
    for r in 0..system.rows() {
        if c >= system.cols() {
            return None
        }
        while system.row(r)[c] == 0.0 {
            c += 1;
        }
        solution.push(system.row(r)[system.cols()-1]);
    }
    Some(solution)
}

fn main() {
    let a = matrix::Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0f64]);

    let b = matrix::Matrix::new(3, 3, vec![1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0, 5.0f64]);

    let c = matrix::Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0f64]);

    let d = matrix::Matrix::new(2, 3, vec![4.0, 3.0, 5.0, 2.0, 4.0, 7.0f64]);

    //let e = matrix::Matrix::new(3, 2, vec![4.0, 3.0, 5.0, 2.0, 4.0, 7.0f64]);

    /*
    to_echelon_form(&mut d);
    d.print();
    println!("");
    to_reduced_echelon_form(&mut d);
    d.print();*/

    println!("{:?}", solve_system_of_linear_equations(a));
    println!("{:?}", solve_system_of_linear_equations(b));
    println!("{:?}", solve_system_of_linear_equations(c));
    println!("{:?}", solve_system_of_linear_equations(d));
    //println!("{:?}", solve_system_of_linear_equations(e));
}
