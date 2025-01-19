use clap::Parser;

#[derive(Parser)]
#[command(name = "determinant CLI", version = "1.0", about = "tool to help you find determinants")]
struct Args {
    matrix: Vec<i64>,
}

fn is_valid_matrix(size: usize) -> bool {
    let root = (size as f64).sqrt();
    root.fract() == 0.0
}

fn find_2x2_det(matrix: &[i64]) -> i64 {
    (matrix[0] * matrix[3]) - (matrix[1] * matrix[2])
}
fn find_3x3_det(matrix: &[i64]) -> i64 {

    let submat1 = [matrix[4], matrix[5], matrix[7], matrix[8]];
    let submat2 = [matrix[3], matrix[5], matrix[6], matrix[8]]; 
    let submat3 = [matrix[3], matrix[4], matrix[6], matrix[7]];

    let a = matrix[0] * (find_2x2_det(&submat1));
    let b = matrix[1] * (find_2x2_det(&submat2));
    let c = matrix[2] * (find_2x2_det(&submat3));
    a - b + c
}

fn main() {
    let args = Args::parse();

    if !is_valid_matrix(args.matrix.len()) {
        println!("error: determinant can only be found for square matrices such as 2x2, 3x3, etc.. ");
        return;
    }

    let determinant = match args.matrix.len() {
        4 => find_2x2_det(&args.matrix),
        9 => find_3x3_det(&args.matrix),
        _ => { 
            println!("what?");
            return;
        }
    };
    
    println!("matrix: {:?}", args.matrix);
    println!("determinant is {}", determinant);
}
