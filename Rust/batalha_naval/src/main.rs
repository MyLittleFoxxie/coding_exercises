// Program to read a naval battle input file
// Pedro and Paulo love to play Naval Battle; despite being good friends, Pedro suspects that Paulo is not playing honestly. To fix this, Pedro decided to use a computer program to verify the game result, but he doesn't know how to program so he asked for your help. The naval battle it's a board game with N lines and M columns. Each position in this board is a square that can contain water or a ship piece. Two squares are neighbors if they have a side in common. If both ship parts are on neighbors then these pieces belong to the same ship. One rule forbids that two different ships share a single ship part (in other words, the squares of two different ship parts are in the same position). Each shot that one player does must be thrown at the board of the other player. The other player tells the other the line and column where he wants to shoot. To destroy a ship, the player must hit all its ship pieces. A player mustn't shoot in the same place twice.
// Write a program that, given the information about the board and the shots given by the player, determines the number of ships that were destroyed.
//
// Input
// The first input line contains two integers N and M (1 ≤ N ≤ 100 e M ≤ 100), representing respectively the number of lines and columns of the board. The next N lines correspond to the board game. Each of these lines contain M characters. Each character indicates the content of the position of the board. If this character is '.', then this position contains water; if it's '#', this position contains a ship piece. The next line contains a number K with the number of the shots given by the player (1 ≤ K ≤ N × M). The next K lines contain two integers L and C, indicating the the line and column of the shot made by the other player (1 ≤ L ≤ N e 1 ≤ C ≤ M).
//
// Output
// Your program must print a single line containing an integer, the number of the ships destroyed. Be careful that when one group of test cases that totalizes 30 points, the ships are composed by one piece (or a square position) and if it totalizes 50 points, each ship is contained exactly within one line.

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_board_size(buf_reader: &mut BufReader<File>) -> io::Result<Vec<i32>> {
    let mut line = String::new();
    buf_reader.read_line(&mut line)?;
    line.split_whitespace()
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn read_board_matrix(buf_reader: &mut BufReader<File>, number_lines: i32) -> io::Result<Vec<Vec<String>>> {
    let mut matrix = Vec::new();
    for _ in 0..number_lines {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let row = line
            .chars()
            .filter(|&c| !c.is_whitespace())
            .map(|c| c.to_string())
            .collect();
        matrix.push(row);
    }
    Ok(matrix)
}

fn read_number_of_shots(buf_reader: &mut BufReader<File>) -> io::Result<i32> {
    let mut line = String::new();
    buf_reader.read_line(&mut line)?;
    line.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn read_shot_matrix(buf_reader: &mut BufReader<File>, number_shots: i32) -> io::Result<Vec<Vec<i32>>> {
    let mut matrix = Vec::new();
    for _ in 0..number_shots {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        matrix.push(row);
    }
    Ok(matrix)
}

fn main() -> io::Result<()> {
    let path = "src/resources/input1.txt";
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let board_size = read_board_size(&mut buf_reader)?;
    let board_matrix = read_board_matrix(&mut buf_reader, board_size[0])?;
    let number_shots = read_number_of_shots(&mut buf_reader)?;
    let shot_matrix = read_shot_matrix(&mut buf_reader, number_shots)?;
    
    println!("Board size: \n{:?}", board_size);
    println!("Board matrix: \n{:?}", board_matrix);
    println!("Number_shots: \n{:?}", number_shots);
    println!("Shots matrix: \n{:?}", shot_matrix);

    Ok(())
}
