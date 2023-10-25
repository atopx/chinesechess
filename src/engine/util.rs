use rand::Rng;

pub fn rank_y(sq: isize) -> isize {
    sq >> 4
}

pub fn file_x(sq: isize) -> isize {
    sq & 15
}

pub fn coord_xy(x: isize, y: isize) -> isize {
    x + (y << 4)
}

pub fn square_fltp(sq: isize) -> usize {
    (254 - sq) as usize
}

pub fn file_fltp(x: isize) -> isize {
    14 - x
}

pub fn mirror_square(sq: isize) -> isize {
    coord_xy(file_fltp(file_x(sq)), rank_y(sq))
}

pub fn square_forward(sq: isize, sd: isize) -> isize {
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

const SHELL_STEPS: [usize; 8] = [0, 1, 4, 13, 40, 121, 364, 1093];

pub fn shell_sort(mvs: &mut Vec<isize>, vls: &mut Vec<isize>) {
    let mut step_level = 1;
    while SHELL_STEPS[step_level] < mvs.len() {
        step_level += 1;
    }
    step_level -= 1;
    while step_level > 0 {
        let step = SHELL_STEPS[step_level];
        for i in 0..mvs.len() {
            let mv_best = mvs[i];
            let vl_best = vls[i];
            let mut j = i as isize - step as isize;
            while j >= 0 && vl_best > vls[j as usize] {
                mvs[(j + step as isize) as usize] = mvs[j as usize];
                vls[(j + step as isize) as usize] = vls[j as usize];
                j -= step as isize;
            }
            mvs[(j + step as isize) as usize] = mv_best;
            vls[(j + step as isize) as usize] = vl_best;
        }
        step_level -= 1;
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

pub fn randf64(value: isize) -> f64 {
    let mut rng = rand::thread_rng();
    let num: f64 = rng.gen_range(0.0..1.0);
    num.floor() * (value as f64)
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
    fn test_movs_iccs() {
        let t = move2iccs(22375);
        assert_eq!(t, "E6-E7");
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

    #[test]
    fn test_shell_sort() {
        let mut mvs = vec![
            22599, 34697, 30615, 34713, 46758, 34728, 46760, 13749, 46773,
        ];
        let mut vls = vec![29, 36, 26, 39, 28, 39, 29, 26, 26];
        shell_sort(&mut mvs, &mut vls);
        let exp_mvs = vec![
            34728, 34713, 34697, 22599, 46760, 46758, 30615, 13749, 46773,
        ];
        let exp_vls = vec![39, 39, 36, 29, 29, 28, 26, 26, 26];
        for i in 0..9 {
            assert_eq!(exp_mvs[i], mvs[i]);
            assert_eq!(exp_vls[i], vls[i]);
        }
    }
}
