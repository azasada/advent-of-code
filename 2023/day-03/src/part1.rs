pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut a: Vec<Vec<char>> = Vec::new(); 
    for line in lines {
        a.push(line.chars().collect());
    }

    let (n, m) = (a.len(), a[0].len());
    let mut is_part_no = vec![vec![false; m]; n];
    for r in 0..n {
        for c in 0..m {
            if !(a[r][c] == '.' || a[r][c].is_digit(10)) {
                if r > 0 {
                    let nr = r - 1;
                    if c > 0 {
                        let nc = c - 1;
                        is_part_no[nr][nc] = a[nr][nc].is_digit(10);
                    }
                    is_part_no[nr][c] = a[nr][c].is_digit(10);
                    if c < n - 1 {
                        let nc = c + 1;
                        is_part_no[nr][nc] = a[nr][nc].is_digit(10);
                    }
                } 
                if c > 0 {
                    let nc = c - 1;
                    is_part_no[r][nc] = a[r][nc].is_digit(10);
                }
                if c < m - 1 {
                    let nc = c + 1;
                    is_part_no[r][nc] = a[r][nc].is_digit(10);
                }
                if r < n - 1 {
                    let nr = r + 1;
                    if c > 0 {
                        let nc = c - 1;
                        is_part_no[nr][nc] = a[nr][nc].is_digit(10);
                    }
                    is_part_no[nr][c] = a[nr][c].is_digit(10);
                    if c < n - 1 {
                        let nc = c + 1;
                        is_part_no[nr][nc] = a[nr][nc].is_digit(10);
                    }
                } 
            }
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 1..m {
            if is_part_no[r][c - 1] && a[r][c].is_digit(10) {
                is_part_no[r][c] = true;
            }
        }
        for c in (0..m-1).rev() {
            if is_part_no[r][c + 1] && a[r][c].is_digit(10) {
                is_part_no[r][c] = true;
            }
        }

        let mut p = 1;
        for c in (0..m).rev() {
            if is_part_no[r][c] {
                ans += p * (a[r][c] as u32 - '0' as u32);
                p *= 10;
            } else {
                p = 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(solve(input), 4361);
    }
}
