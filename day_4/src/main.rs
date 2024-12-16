use std::fs;

fn main() {
    let data = fs::read_to_string("src/input.txt").unwrap();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        matrix.push(line.chars().collect());
    }

    let mut occurences = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                // UP
                if row > 2 && check_up(&matrix, row - 1, col, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // UP RIGHT
                if row > 2 && check_up_right(&matrix, row - 1, col + 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // RIGHT
                if col < matrix[row].len() - 3 && check_right(&matrix, row, col + 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // DOWN RIGHT
                if col < matrix[row].len() - 3 && row < matrix.len() - 3 && check_down_right(&matrix, row + 1, col + 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // DOWN
                if row < matrix.len() - 3 && check_down(&matrix, row + 1, col, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // DOWN LEFT
                if col > 2 && row < matrix.len() - 3 && check_down_left(&matrix, row + 1, col - 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // LEFT
                if col > 2 && check_left(&matrix, row, col - 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }

                // UP LEFT
                if row > 2 && col > 2 && check_up_left(&matrix, row - 1, col - 1, &['M', 'A', 'S']) {
                    occurences += 1;
                }
            }
        }
    }

    println!("{}", occurences);

    let mut occurences = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if row >= 1 && col >= 1 && row <= matrix.len() - 2 && col <= matrix[row].len() - 2 && matrix[row][col] == 'A' {
                if check_diagonal_upleft_rightdown(&matrix, row, col) && check_diagonal_downleft_upright(&matrix, row, col) {
                    occurences += 1;
                }
            }
        }
    }

    println!("{}", occurences);

}

fn check_diagonal_upleft_rightdown(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if (matrix[row - 1][col - 1] == 'M' && matrix[row + 1][col + 1] == 'S')
        || (matrix[row - 1][col - 1] == 'S' && matrix[row + 1][col + 1] == 'M') {
        return true;
    }

    false
}

fn check_diagonal_downleft_upright(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if (matrix[row + 1][col - 1] == 'M' && matrix[row - 1][col + 1] == 'S')
        || (matrix[row + 1][col - 1] == 'S' && matrix[row - 1][col + 1] == 'M') {
        return true;
    }

    false
}

fn check_up(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == 0 {
        return false;
    }

    check_up(matrix, row - 1, col, &chars[1..])
}

fn check_up_right(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if col > matrix[row].len() - 1 {
        return false;
    }

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == 0 || col == matrix[row].len() - 1{
        return false;
    }

    check_up_right(matrix, row - 1, col + 1, &chars[1..])
}

fn check_right(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if col == matrix[row].len() - 1 {
        return false;
    }

    check_right(matrix, row, col + 1, &chars[1..])
}

fn check_down_right(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if row > matrix.len() - 1 || col > matrix[row].len() - 1{
        return false;
    }

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == matrix.len() - 1 || col == matrix[row].len() - 1{
        return false;
    }

    check_down_right(matrix, row + 1, col + 1, &chars[1..])
}

fn check_down(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == matrix.len() - 1 {
        return false;
    }

    check_down(matrix, row + 1, col, &chars[1..])
}

fn check_down_left(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if row > matrix.len() - 1 {
        return false;
    }

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == matrix.len() - 1 || col == 0{
        return false;
    }

    check_down_left(matrix, row + 1, col - 1, &chars[1..])
}

fn check_left(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if col == 0 {
        return false;
    }

    check_left(matrix, row, col - 1, &chars[1..])
}

fn check_up_left(matrix: &Vec<Vec<char>>, row: usize, col: usize, chars: &[char]) -> bool {
    println!("{} {} {:?}", row, col, &chars);
    let char = chars[0];

    if matrix[row][col] != char {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    if row == 0 || col == 0 {
        return false;
    }

    check_up_left(matrix, row - 1, col - 1, &chars[1..])
}