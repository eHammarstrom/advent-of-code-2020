use std::io::prelude::*;
use std::io;

fn input_to_string<R: Read>(r: R) -> io::Result<String> {
    let mut reader = io::BufReader::new(r);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    Ok(data)
}

fn input_to_matrix(input: &str, (col_len, row_len): (usize, usize)) -> Vec<Vec<bool>> {
    let mut mat: Vec<Vec<bool>> = vec![Vec::new(); col_len];
    let mut input_index = 0;

    for c in input.chars() {
        if c == '\n' {
            continue;
        }

        let tree = c == '#';
        let col: &mut Vec<bool> = &mut mat[input_index % col_len];
        input_index += 1;
        col.push(tree)
    }

    println!("col {}, row {}", mat.len(), mat[0].len());
    assert_eq!(mat.len(), col_len);
    assert_eq!(mat[0].len(), row_len);

    mat
}

fn count_row_len(input: &str) -> (usize, usize) {
    let mut row_len = 0;
    let mut col_len = 0;

    for c in input.chars() {
        if c != '\n' {
            row_len += 1;
        } else if col_len == 0 {
            col_len = row_len;
        }
    }

    (col_len, row_len / col_len)
}

fn traverse_matrix((col_move, row_move): (usize, usize), matrix: &mut Vec<Vec<bool>>) -> u32 {
    let bottom = matrix.len();
    let mut level = 0;
    let mut encountered_trees = 0;
    let mut matrix_expand_index = 0;

    while level != bottom {
        /* Grow matrix if needed */
        if let None = matrix.get(col_move * level) {
            for _ in 0..=col_move {
                let expansion = matrix[matrix_expand_index].to_owned();
                matrix.push(expansion);
                matrix_expand_index += 1;
            }
            continue;
        }
        println!("pos ({}, {})", col_move * level, row_move * level);

        let col = &matrix[col_move * level];
        let row = col[row_move * level];

        if row {
            encountered_trees += 1;
        }
        level += row_move;
    }

    encountered_trees
}

fn main() -> io::Result<()> {
    let mut input = input_to_string(io::stdin())?;
    let (col_len, row_len) = count_row_len(&input);
    println!("col_len {} row_len {}", col_len, row_len);
    let mut mat = input_to_matrix(&mut input, (col_len, row_len));
    let (right, down) = (3, 1);
    let encountered_trees = traverse_matrix((right, down), &mut mat);

    println!("Solution A: {}", encountered_trees);

    Ok(())
}
