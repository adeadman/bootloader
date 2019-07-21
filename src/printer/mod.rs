#[cfg(feature = "vga_320x200")]
pub use self::vga_320x200::*;

#[cfg(feature = "vesa_800x600")]
pub use self::vesa_800x600::*;

#[cfg(not(any(feature = "vga_320x200", feature = "vesa_800x600")))]
pub use self::vga_text_80x25::*;

#[cfg(feature = "vga_320x200")]
mod vga_320x200;

#[cfg(feature = "vesa_800x600")]
mod vesa_800x600;

#[cfg(not(any(feature = "vga_320x200", feature = "vesa_800x600")))]
mod vga_text_80x25;
