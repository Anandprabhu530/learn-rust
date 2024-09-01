// Transpose Matrix
// matrix: ( 1.1 1.2 )
//         ( 2.1 2.2 )
// Transpose: ( 1.1 2.1 )
//            ( 1.2 2.2 )

fn main(){
    let matrix = ((1,2),(3,4));
    println!("Matrix:\n{:?}", matrix);
    println!("Transpose:\n{:?}", transpose_matrix(matrix));
}

fn transpose_matrix(matrix:((i32,i32),(i32,i32)))->((i32,i32),(i32,i32)){
    let ((a1,a2),(a3,a4)) = matrix;
    ((a1,a3),(a2,a4))
}