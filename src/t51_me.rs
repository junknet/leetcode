pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut board = Board::new(n as usize);
    board.dfs(0);
    board.result
}

struct Board {
    n: usize,
    col: Vec<bool>,
    left_side: Vec<bool>,
    right_side: Vec<bool>,
    queen_pos: Vec<usize>,
    result: Vec<Vec<String>>,
}

impl Board {
    fn new(n: usize) -> Self {
        let (mut col, mut left_side, mut right_side, mut queen_pos) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        col.resize(n, false);
        left_side.resize(n << 1, false);
        right_side.resize(n << 1, false);
        queen_pos.resize(n, 0);
        Board {
            n,
            col,
            left_side,
            right_side,
            queen_pos,
            result: Vec::new(),
        }
    }

    fn dfs(&mut self, row: usize) {
        if row == self.n {
            let mut tmp = Vec::with_capacity(self.n);
            self.queen_pos.iter().for_each(|&pos| {
                tmp.push(".".repeat(pos) + "Q" + &".".repeat(self.n - 1 - pos));
            });
            self.result.push(tmp);
            return;
        }
        for col in 0..self.n {
            if !self.col[col]
                && !self.left_side[col + row]
                && !self.right_side[self.n + row - col]
            {
                self.queen_pos[row] = col;
                self.col[col] = true;
                self.left_side[col + row] = true;
                self.right_side[self.n + row - col] = true;
                self.dfs(row + 1);
                self.col[col] = false;
                self.left_side[col + row] = false;
                self.right_side[self.n + row - col] = false;
            }
        }
    }
}
