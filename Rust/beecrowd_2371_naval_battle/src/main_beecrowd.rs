use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let board_size = read_board_size(&mut lines)?;
    let mut board_matrix = read_board_matrix(&mut lines, board_size[0])?;
    let number_shots = read_number_of_shots(&mut lines)?;
    let shot_matrix = read_shot_matrix(&mut lines, number_shots)?;

    for shot in shot_matrix {
        let row = (shot[0] - 1) as usize;
        let col = (shot[1] - 1) as usize;
        if board_matrix[row][col] == '#' {
            board_matrix[row][col] = 'X';
        }
    }

    let mut destroyed_ship_count = 0;
    let mut visited = vec![vec![false; board_size[1] as usize]; board_size[0] as usize];
    for i in 0..board_size[0] as usize {
        for j in 0..board_size[1] as usize {
            if board_matrix[i][j] == 'X' && !visited[i][j] {
                if bfs(&board_matrix, &mut visited, i, j) {
                    destroyed_ship_count += 1;
                }
            }
        }
    }

    println!("{}", destroyed_ship_count);
    Ok(())
}

fn read_board_size(lines: &mut std::io::Lines<std::io::StdinLock>) -> io::Result<Vec<usize>> {
    let line = lines.next().unwrap()?;
    Ok(line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect())
}

fn read_board_matrix(
    lines: &mut std::io::Lines<std::io::StdinLock>,
    number_lines: usize,
) -> io::Result<Vec<Vec<char>>> {
    let mut matrix = Vec::new();
    for _ in 0..number_lines {
        let line = lines.next().unwrap()?;
        let row = line.trim().chars().collect();
        matrix.push(row);
    }
    Ok(matrix)
}

fn read_number_of_shots(lines: &mut std::io::Lines<std::io::StdinLock>) -> io::Result<usize> {
    let line = lines.next().unwrap()?;
    Ok(line.trim().parse::<usize>().unwrap())
}

fn read_shot_matrix(
    lines: &mut std::io::Lines<std::io::StdinLock>,
    number_shots: usize,
) -> io::Result<Vec<Vec<usize>>> {
    let mut matrix = Vec::new();
    for _ in 0..number_shots {
        let line = lines.next().unwrap()?;
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        matrix.push(row);
    }
    Ok(matrix)
}

fn bfs(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_i: usize,
    start_j: usize,
) -> bool {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;

    while let Some((row, col)) = queue.pop_front() {
        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row >= 0
                && new_col >= 0
                && (new_row as usize) < board.len()
                && (new_col as usize) < board[0].len()
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if !visited[new_row][new_col] {
                    if board[new_row][new_col] == 'X' {
                        visited[new_row][new_col] = true;
                        queue.push_back((new_row, new_col));
                    } else if board[new_row][new_col] == '#' {
                        return false;
                    }
                }
            }
        }
    }
    true
}
