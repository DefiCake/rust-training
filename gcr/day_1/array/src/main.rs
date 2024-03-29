fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
  let mut result: [[i32; 3]; 3] = Default::default();

  for i in 0..3 {
    for j in 0..3 {
      result[i][j] = matrix[j][i];
    }
  }

  result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
  for i in 0..3 {
    print!("|");
    for j in 0..3 {
      print!(" {} ", matrix[i][j]);
    }
    println!("|");
  }
}

fn main() {
  let matrix = [
    [101, 102, 103], // <-- the comment makes rustfmt add a newline
    [201, 202, 203],
    [301, 302, 303],
  ];

  println!("matrix:");
  pretty_print(&matrix);

  let transposed = transpose(matrix);
  println!("transposed:");
  pretty_print(&transposed);
}

#[test]
fn test_transpose() {
  let matrix = [
    [1, 4, 7], //
    [2, 5, 8],
    [3, 6, 9],
  ];

  let transposed_matrix = [
    [1, 2, 3], //
    [4, 5, 6],
    [7, 8, 9],
  ];

  assert_eq!(transpose(matrix), transposed_matrix);
}
