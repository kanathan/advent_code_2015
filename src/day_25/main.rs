

fn main() {
    let (row, col) = (2947, 3029);

    println!("{}",find_code(row, col));
}


fn find_code(row: usize, col: usize) -> u64 {
    let mut code = 20151125;
    let (mut cur_row, mut cur_col) = (1, 1);

    while cur_row != row || cur_col != col {
        code = (code * 252533) % 33554393;
        cur_row -= 1;
        cur_col += 1;
        if cur_row == 0 {
            cur_row = cur_col;
            cur_col = 1;
        }
    }

    code
}


#[cfg(test)]
mod test {
    use crate::find_code;

    


    #[test]
    fn test1() {
        assert_eq!(find_code(6,6), 27995004);
    }
}