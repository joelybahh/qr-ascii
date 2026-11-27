use crate::bits::BitBuffer;

mod bits;
mod encode;
mod gf;

/** QR CONSTANTS */
const QR_W_H: usize = 33;
const QUIET_ZONE: usize = 4;

/** FINDER CONSTANTS */
const FINDER_W: usize = 7;
const FINDER_RES_W: usize = 8;
const PDP_1: [bool; 7] = [true; 7];
const PDP_2: [bool; 7] = [true, false, false, false, false, false, true];
const PDP_3: [bool; 7] = [true, false, true, true, true, false, true];

const FINDER: [[bool; 7]; 7] = [PDP_1, PDP_2, PDP_3, PDP_3, PDP_3, PDP_2, PDP_1];
const FINDER_RES: [[bool; 8]; 8] = [[true; 8]; 8];

/** ALIGNMENT CONSTANTS */
const ALIGNMENT_W: usize = 5;
const ADP_1: [bool; 5] = [true; 5];
const ADP_2: [bool; 5] = [true, false, false, false, true];
const ADP_3: [bool; 5] = [true, false, true, false, true];

const ALIGNMENT: [[bool; 5]; 5] = [ADP_1, ADP_2, ADP_3, ADP_2, ADP_1];
const ALIGNMENT_RES: [[bool; 5]; 5] = [[true; 5]; 5];
const ALIGNMENT_CENTER: usize = QR_W_H - 7;

/** DARK MODULE CONSTANTS */
const DARK_MODULE_ROW: usize = QR_W_H - FINDER_RES_W; // 25
const DARK_MODULE_COL: usize = FINDER_RES_W; // 8

struct Qr {
    dark: Vec<Vec<bool>>,
    reserved: Vec<Vec<bool>>,
}

impl Qr {
    fn new() -> Qr {
        Qr {
            dark: vec![vec![false; QR_W_H]; QR_W_H],
            reserved: vec![vec![false; QR_W_H]; QR_W_H],
        }
    }

    fn setup(&mut self) {
        draw_finder(&mut self.dark);
        draw_alignment(&mut self.dark);
        draw_timing(&mut self.dark);
        draw_dark_module(&mut self.dark);

        reserve_areas(&mut self.reserved);
    }

    fn render(&self) {
        render_qr(&self.dark);
    }

    #[cfg(test)]
    fn empty_modules(&self) -> usize {
        return self.reserved.iter().flatten().filter(|&&r| !r).count();
    }
}

fn main() {
    let qr = &mut Qr::new();
    qr.setup();

    qr.render();
}

fn render_module(state: bool) {
    if state {
        print!("  ");
    } else {
        print!("██");
    }
}

fn render_row(states: &Vec<bool>) {
    for state in states {
        render_module(*state);
    }
}

fn render_qr(states: &Vec<Vec<bool>>) {
    render_qz_top_bottom();
    render_grid(states);
    render_qz_top_bottom();
}

fn render_grid(states: &Vec<Vec<bool>>) {
    for state in states {
        render_qz_left_right();
        render_row(state);
        render_qz_left_right();
        println!();
    }
}

fn render_qz_top_bottom() {
    render_grid(&mut vec![vec![false; QR_W_H]; QUIET_ZONE]);
}

fn render_qz_left_right() {
    render_row(&mut vec![false; QUIET_ZONE]);
}

fn draw_finder(qr: &mut Vec<Vec<bool>>) {
    overlay(qr, &FINDER, 0, 0);
    overlay(qr, &FINDER, QR_W_H - FINDER_W, 0);
    overlay(qr, &FINDER, 0, QR_W_H - FINDER_W);
}

fn draw_alignment(qr: &mut Vec<Vec<bool>>) {
    overlay(
        qr,
        &ALIGNMENT,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
    );
}

fn draw_timing(qr: &mut Vec<Vec<bool>>) {
    let fixed = FINDER_W - 1; // timing starts 6 across or 6 down.

    // for every number between 8 -> 24 (This is the index start/finish for the loop)
    for i in FINDER_RES_W..(QR_W_H - FINDER_RES_W) {
        let dark = i % 2 == 0;
        qr[fixed][i] = dark; // horizontal
        qr[i][fixed] = dark; // vertical
    }
}

fn draw_dark_module(qr: &mut Vec<Vec<bool>>) {
    qr[DARK_MODULE_ROW][DARK_MODULE_COL] = true;
}

fn reserve_areas(reserved: &mut Vec<Vec<bool>>) {
    // reserve finder
    overlay(reserved, &FINDER_RES, 0, 0);
    overlay(reserved, &FINDER_RES, QR_W_H - FINDER_RES_W, 0);
    overlay(reserved, &FINDER_RES, 0, QR_W_H - FINDER_RES_W);

    // reserve alignment
    overlay(
        reserved,
        &ALIGNMENT_RES,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
    );
    // reserve timing
    let fixed = FINDER_W - 1;
    for i in FINDER_RES_W..(QR_W_H - FINDER_RES_W) {
        reserved[fixed][i] = true; // horizontal
        reserved[i][fixed] = true; // vertical
    }

    // reserve dark module
    reserved[DARK_MODULE_ROW][DARK_MODULE_COL] = true; // TODO: Same as above

    // reserve format info
    // first loop mirror trick
    let fixed = FINDER_RES_W;
    for i in 0..=DARK_MODULE_COL {
        // TODO: Same as above
        reserved[i][fixed] = true;
        reserved[fixed][i] = true;
    }
    // second loop
    for i in DARK_MODULE_ROW..=QR_W_H - 1 {
        // TODO: Same as above
        reserved[i][fixed] = true;
        reserved[fixed][i] = true;
    }
}

fn overlay<const H: usize, const W: usize>(
    qr: &mut Vec<Vec<bool>>,
    shape: &[[bool; W]; H],
    offset_x: usize,
    offset_y: usize,
) {
    for (y, row) in shape.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            qr[offset_y + y][offset_x + x] = *cell;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreserved_is_807_after_setup() {
        let mut qr = Qr::new();
        qr.setup();
        assert_eq!(qr.empty_modules(), 807);
    }
}
