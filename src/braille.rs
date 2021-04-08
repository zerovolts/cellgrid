use crate::{coord::Coord, grid::Grid, rect::Rect};
use std::char;

pub fn braillify_grid(bitgrid: Grid<bool>) -> Grid<char> {
    // Assume the input grid has dimensions that are a multiple of (2, 4).
    Grid::<char>::with_generator(
        Rect::new((bitgrid.bounds.width() / 2, bitgrid.bounds.height() / 4)),
        |coord: Coord| {
            // Get a 2x4 rect of cells from the bitgrid and reduce them into a
            // single char.
            let top_left = Coord::new(coord.x * 2, coord.y * 4);
            let rect = Rect::with_corners(top_left, top_left + Coord::new(2, 4));
            let bits = bitgrid
                .selection_iter(rect.iter())
                .map(|cell| if let Ok(cell) = cell { *cell.1 } else { false })
                .collect::<Vec<bool>>();
            byte_to_braille(bools_to_byte(&bits[0..8]))
        },
    )
}

/// Mapping from this library's dot order to Braille's dot order.
///
/// Library order - ⣾ ⣷ ⣽ ⣯ ⣻ ⣟ ⢿ ⡿ (row, then column)
/// Braille order - ⣾ ⣽ ⣻ ⣷ ⣯ ⣟ ⢿ ⡿ (column for the first 3 rows, then row)
const DOT_ORDER_MAP: [u8; 8] = [0, 3, 1, 4, 2, 5, 6, 7];

/// `bools` must have length 8.
fn bools_to_byte(bools: &[bool]) -> u8 {
    let mut total = 0;
    for i in 0..8 {
        if bools[i] {
            total ^= 1 << DOT_ORDER_MAP[i];
        }
    }
    total
}

/// Unicode code point for the first Braille character.
/// Decimal: 10240
const BRAILLE_UNICODE_OFFSET: u32 = 0x2800;

fn byte_to_braille(byte: u8) -> char {
    // SAFETY: All 256 possible byte values (when offset) map to valid unicode
    // characters.
    unsafe { char::from_u32_unchecked(byte as u32 + BRAILLE_UNICODE_OFFSET) }
}
