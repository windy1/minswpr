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
}
