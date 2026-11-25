/** QR CONSTANTS */
const QR_W_H: usize = 33;
const QUIET_ZONE_PADDING: usize = 4;
const QR_FULL_W_H: usize = QR_W_H + QUIET_ZONE_PADDING * 2;

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

struct Qr {
    dark: Vec<Vec<bool>>,
    reserved: Vec<Vec<bool>>,
}

impl Qr {
    fn new() -> Qr {
        Qr {
            dark: vec![vec![false; QR_FULL_W_H]; QR_FULL_W_H],
            reserved: vec![vec![false; QR_FULL_W_H]; QR_FULL_W_H],
        }
    }
}

fn main() {
    let qr = &mut Qr::new();
    draw_finder(&mut qr.dark);
    draw_alignment(&mut qr.dark);
    draw_timing(&mut qr.dark, QUIET_ZONE_PADDING);
    draw_dark_module(&mut qr.dark, QUIET_ZONE_PADDING);

    reserve_areas(&mut qr.reserved);

    render_grid(&mut qr.dark);
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
    println!();
}

fn render_grid(states: &Vec<Vec<bool>>) {
    for state in states {
        render_row(state);
    }
}

fn draw_finder(qr: &mut Vec<Vec<bool>>) {
    overlay(qr, &FINDER, 0, 0, QUIET_ZONE_PADDING);
    overlay(qr, &FINDER, QR_W_H - FINDER_W, 0, QUIET_ZONE_PADDING);
    overlay(qr, &FINDER, 0, QR_W_H - FINDER_W, QUIET_ZONE_PADDING);
}

fn draw_alignment(qr: &mut Vec<Vec<bool>>) {
    overlay(
        qr,
        &ALIGNMENT,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        QUIET_ZONE_PADDING,
    );
}

fn draw_timing(qr: &mut Vec<Vec<bool>>, padding: usize) {
    let fixed = FINDER_W - 1 + padding; // timing starts 6 across or 6 down.

    // for every number between 8 -> 24 (This is the index start/finish for the loop)
    for i in (FINDER_W + 1)..=(QR_W_H - FINDER_W - 2) {
        let dark = i % 2 == 0;
        qr[fixed][i + padding] = dark; // horizontal
        qr[i + padding][fixed] = dark; // vertical
    }
}

fn draw_dark_module(qr: &mut Vec<Vec<bool>>, padding: usize) {
    qr[25 + padding][8 + padding] = true;
}

fn reserve_areas(reserved: &mut Vec<Vec<bool>>) {
    // reserve finder
    overlay(reserved, &FINDER_RES, 0, 0, QUIET_ZONE_PADDING);
    overlay(
        reserved,
        &FINDER_RES,
        QR_W_H - FINDER_RES_W,
        0,
        QUIET_ZONE_PADDING,
    );
    overlay(
        reserved,
        &FINDER_RES,
        0,
        QR_W_H - FINDER_RES_W,
        QUIET_ZONE_PADDING,
    );

    // reserve alignment
    overlay(
        reserved,
        &ALIGNMENT_RES,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        ALIGNMENT_CENTER - ALIGNMENT_W / 2,
        QUIET_ZONE_PADDING,
    );
    // reserve timing
    let fixed = FINDER_W - 1 + QUIET_ZONE_PADDING; // timing starts 6 across or 6 down.
    for i in (FINDER_W + 1)..=(QR_W_H - FINDER_W - 2) {
        reserved[fixed][i + QUIET_ZONE_PADDING] = true; // horizontal
        reserved[i + QUIET_ZONE_PADDING][fixed] = true; // vertical
    }

    // reserve dark module
    reserved[25 + QUIET_ZONE_PADDING][8 + QUIET_ZONE_PADDING] = true;

    // reserve format info
    // first loop mirror trick
    let fixed = FINDER_RES_W + QUIET_ZONE_PADDING;
    for i in 0..=8 {
        reserved[i + QUIET_ZONE_PADDING][fixed] = true;
        reserved[fixed][i + QUIET_ZONE_PADDING] = true;
    }
    // second loop
    for i in 25..=32 {
        reserved[i + QUIET_ZONE_PADDING][fixed] = true;
        reserved[fixed][i + QUIET_ZONE_PADDING] = true;
    }
}

fn overlay<const H: usize, const W: usize>(
    qr: &mut Vec<Vec<bool>>,
    shape: &[[bool; W]; H],
    offset_x: usize,
    offset_y: usize,
    padding: usize,
) {
    for (y, row) in shape.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            qr[offset_y + y + padding][offset_x + x + padding] = *cell;
        }
    }
}
