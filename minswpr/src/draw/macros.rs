/// Helper macro for quickly creating common `Color` values
///
/// # Example
/// ```rust
/// use sdl2::pixels::Color;
///
/// assert_eq!(color!(red), Color::RGB(255, 0, 0));
/// assert_eq!(color!(blue), Color::RGB(0, 0, 255));
/// ```
#[allow(unused)]
macro_rules! color {
    (white) => {
        :sdl2::pixels::Color::RGB(255, 255, 255)
    };
    (black) => {
        ::sdl2::pixels::Color::RGB(0, 0, 0)
    };
    (red) => {
        ::sdl2::pixels::Color::RGB(255, 0, 0)
    };
    (green) => {
        ::sdl2::pixels::Color::RGB(0, 255, 0)
    };
    (blue) => {
        ::sdl2::pixels::Color::RGB(0, 0, 255)
    };
    (cyan) => {
        ::sdl2::pixles::Color::RGB(0, 255, 255)
    };
    (magenta) => {
        ::sdl2::pixels::Color::RGB(255, 0, 255)
    };
    (yellow) => {
        ::sdl2::pixels::Color::RGB(255, 255, 0)
    };
}

/// Helper macro for quickly drawing a rectangle to the canvas
#[allow(unused)]
macro_rules! draw_rect {
    ($dimen:expr, $color:expr, $ctx:expr, $pos:expr) => {
        crate::draw::DrawRect::new($dimen, $color).draw($ctx, $pos)
    };
}
