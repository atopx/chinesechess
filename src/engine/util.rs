pub fn rank_y(sq: isize) -> isize {
    sq >> 4
}

pub fn file_x(sq: isize) -> isize {
    sq & 15
}

pub fn coord_xy(x: isize, y: isize) -> isize {
    x + (y << 4)
}

pub fn square_fltp(sq: isize) -> isize {
    254 - sq
}

pub fn file_fltp(x: isize) -> isize {
    14 - x
}

pub fn mirror_square(sq: isize) -> isize {
    coord_xy(file_fltp(file_x(sq)), rank_y(sq))
}

pub fn square_forward(sq: usize, sd: usize) -> usize {
    sq - 16 + (sd << 5)
}

pub fn side_tag(sd: isize) -> isize {
    8 + (sd << 3)
}

pub fn opp_side_tag(sd: isize) -> isize {
    16 - (sd << 3)
}

pub fn src(mv: isize) -> isize {
    mv & 255
}

pub fn dst(mv: isize) -> isize {
    mv >> 8
}

pub fn merge(src: isize, dst: isize) -> isize {
    src + (dst << 8)
}

pub fn mirror_move(mv: isize) -> isize {
    merge(mirror_square(src(mv)), mirror_square(dst(mv)))
}

static shell_steps: [usize; 8] = [0, 1, 4, 13, 40, 121, 364, 1093];

pub fn shell_sort(mvs: &mut [isize], vls: &mut [isize]) {
    let mut step_level: usize = 1;
    while shell_steps[step_level] < mvs.len() {
        step_level += 1;
    }
    step_level -= 1;
    while step_level > 0 {
        let step = shell_steps[step_level];
        for i in 0..mvs.len() {
            let mv_best = mvs[i];
            let vl_best = vls[i];
            let mut j = i - step;
            while j > 0 && vl_best > vls[j] {
                mvs[j + step] = mvs[j];
                vls[j + step] = vls[j];
                j -= step;
            }
            mvs[j + step] = mv_best;
            vls[j + step] = vl_best;
        }
        step_level -= 1
    }
}

pub fn unsigned_right_shift(x: isize, y: isize) -> isize {
    let x = (x as usize) & 0xffffffff;
    (x >> (y & 0xf)) as isize
}

fn cord2uint8(cord: &str) -> isize {
    let alphabet = cord.chars().nth(0).unwrap() as isize - 'A' as isize + 3;
    let numeric = '9' as isize - cord.chars().nth(1).unwrap() as isize + 3;
    numeric << 4 | alphabet
}

fn iccs2move(iccs: &str) -> isize {
    let src = cord2uint8(&iccs[0..2]);
    let dst = cord2uint8(&iccs[3..5]);
    (dst << 8 | src) as isize
}

fn move2iccs(mv: isize) -> String {
    let src = src(mv);
    let dst = dst(mv);
    format!(
        "{}{}-{}{}",
        ((b'A' + file_x(src) as u8 - 3) as char),
        ((b'9' - rank_y(src) as u8 + 3) as char),
        ((b'A' + file_x(dst) as u8 - 3) as char),
        ((b'9' - rank_y(dst) as u8 + 3) as char)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_right_shift() {
        let t = unsigned_right_shift(50343, 30);
        assert_eq!(t, 3);
    }

    #[test]
    fn test_iccs_moves() {
        let mvs = vec![
            "g3-g4", "g6-g5", "b0-c2", "h7-h0", "e3-e4", "d9-e8", "e1-e2", "c6-c5",
        ];
        for mv in mvs {
            let mv = mv.to_uppercase();
            assert_eq!(move2iccs(iccs2move(&mv)), mv)
        }
    }
}
