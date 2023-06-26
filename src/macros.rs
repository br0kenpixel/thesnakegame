#[macro_export]
macro_rules! set_pixel {
    ($ctx: expr, $x: expr, $y: expr, $fg: ident, $bg: ident, $character: literal) => {
        $ctx.set($x, $y, $fg, $bg, to_cp437($character));
    };

    // If the background is not specified, default to black
    ($ctx: expr, $x: expr, $y: expr, $fg: ident, $character: literal) => {
        $ctx.set($x, $y, $fg, BLACK, to_cp437($character));
    };
}
