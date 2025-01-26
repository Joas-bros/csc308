use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight};

/// Constants for the usage of the [`noto_sans_mono_bitmap`] crate.
pub mod font_constants {
    use super::*;

    /// Height of each character raster.
    /// The font size is approximately 0.84% of this value. This serves as the line height
    /// that enables multiple characters to appear optically aligned on one line naturally.
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    /// The width of each single symbol of the monospace font.
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    /// Backup character used if a desired symbol is not available in the font.
    /// The '\u{0009}' character requires the feature "unicode-specials".
    pub const BACKUP_CHAR: char = '\u{0009}';

    /// Default font weight for the monospace font.
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

    /// Backspace character.
    pub const BACKSPACE: char = '\u{0008}';
}
