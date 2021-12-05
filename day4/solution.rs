fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let numbers = lines[0].split(",");
    let mut i = 2;
    let mut boards : Vec<[[i32; 5]; 5]> = Vec::new();
    while i < lines.len() {
        let mut board = [[0; 5]; 5];
        for j in i..i+5 {
            let mut k = 0;
            for num in lines[j].split_whitespace() {
                board[j - i][k] = num.parse::<i32>().unwrap();
                k += 1;
            }
        }
        boards.push(board);
        i += 6
    }
    let mut board_scores = Vec::new();
    for num in numbers {
        let num_int = num.parse::<i32>().unwrap(); 
        for board in boards.iter_mut() {
            // go through and find number
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == num_int {
                        board[i][j] = 0;
                    }
                }
            }
            // check for bingo
            let mut bingo = false;
            for i in 0..5 {
                if board[i] == [0, 0, 0, 0, 0] {
                    bingo = true;
                    break;
                }
                bingo = board[0][i] == 0 &&
                        board[1][i] == 0 &&
                        board[2][i] == 0 &&
                        board[3][i] == 0 &&
                        board[4][i] == 0;
                if bingo {
                    break;
                }

            }
            if bingo {
                let mut result = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        if board[i][j] != 0 {
                            result += board[i][j];
                        }
                    }
                }
                if result != 0 {
                    board_scores.push(result*num_int);
                    for i in 0..5 {
                        for j in 0..5 {
                            board[i][j] = 0;
                        }
                    }
                }
            
            }
        }
    }
    board_scores.retain(|&x| x != 0);
    println!("{}", board_scores[0]);
    println!("{}", board_scores[board_scores.len() - 1]);
}
