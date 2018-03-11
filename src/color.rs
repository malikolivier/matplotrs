//!
//! A library providing simple `Color` and `Gradient` types along with useful transformations and
//! presets.
//!
//!
//! Inspiration taken from [elm-lang's color module]
//! (https://github.com/elm-lang/core/blob/62b22218c42fb8ccc996c86bea450a14991ab815/src/Color.elm)
//!
//!
//! Module for working with colors. Includes [RGB](https://en.wikipedia.org/wiki/RGB_color_model)
//! and [HSL](http://en.wikipedia.org/wiki/HSL_and_HSV) creation, gradients and built-in names.
//!

/// RGBA Color
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);


impl Color {
    /// Create RGB colors with an alpha component for transparency.
    /// The alpha component is specified with numbers between 0 and 1.
    pub fn rgba<R: Into<f64>, G: Into<f64>, B: Into<f64>, A: Into<f64>>(
        r: R,
        g: G,
        b: B,
        a: A,
    ) -> Self {
        Color(r.into(), g.into(), b.into(), a.into())
    }

    /// Create RGB colors from numbers between 0.0 and 1.0.
    pub fn rgb<R: Into<f64>, G: Into<f64>, B: Into<f64>>(r: R, g: G, b: B) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Produce a gray based on the input. 0.0 is white, 1.0 is black.
    pub fn grayscale<P: Into<f64>>(p: P) -> Self {
        let val = 1.0 - p.into();
        Self::rgb(val, val, val)
    }

    /// Return the red value.
    pub fn red(&self) -> f64 {
        self.0
    }

    /// Return the green value.
    pub fn green(&self) -> f64 {
        self.1
    }

    /// Return the blue value.
    pub fn blue(&self) -> f64 {
        self.2
    }

    /// Set the red value.
    pub fn set_red<R: Into<f64>>(&mut self, r: R) {
        self.0 = r.into();
    }

    /// Set the green value.
    pub fn set_green<G: Into<f64>>(&mut self, g: G) {
        self.1 = g.into();
    }

    /// Set the blue value.
    pub fn set_blue<B: Into<f64>>(&mut self, b: B) {
        self.2 = b.into();
    }

    pub fn bytes_rgb(&self) -> [u8; 3] {
        [f64_to_byte(self.0), f64_to_byte(self.1), f64_to_byte(self.2)]
    }
}

fn f64_to_byte(x: f64) -> u8 {
    (x * 255.0) as u8
}

/// Built-in colors.
///
/// These colors come from the
/// [Tango palette](http://tango.freedesktop.org/Tango_Icon_Theme_Guidelines) which provides
/// aesthetically reasonable defaults for colors. Each color also comes with a light and dark
/// version.

macro_rules! make_color {
    ($r:expr, $g:expr, $b:expr) => ( Color($r as f64 / 255.0,
                                           $g as f64 / 255.0,
                                           $b as f64 / 255.0, 1.0));
    ($r:expr, $g:expr, $b:expr, $a:expr) => ( Color($r as f64 / 255.0,
                                                    $g as f64 / 255.0,
                                                    $b as f64 / 255.0,
                                                    $a as f64 / 255.0));
}

/// Scarlet Red - Light - #EF2929
pub const LIGHT_RED: Color = make_color!(239, 41, 41);
/// Scarlet Red - Regular - #CC0000
pub const RED: Color = make_color!(204, 0, 0);
/// Scarlet Red - Dark - #A30000
pub const DARK_RED: Color = make_color!(164, 0, 0);

/// Orange - Light - #FCAF3E
pub const LIGHT_ORANGE: Color = make_color!(252, 175, 62);
/// Orange - Regular - #F57900
pub const ORANGE: Color = make_color!(245, 121, 0);
/// Orange - Dark - #CE5C00
pub const DARK_ORANGE: Color = make_color!(206, 92, 0);

/// Butter - Light - #FCE94F
pub const LIGHT_YELLOW: Color = make_color!(252, 233, 79);
/// Butter - Regular - #EDD400
pub const YELLOW: Color = make_color!(237, 212, 0);
/// Butter - Dark - #C4A000
pub const DARK_YELLOW: Color = make_color!(196, 160, 0);

/// Chameleon - Light - #8AE234
pub const LIGHT_GREEN: Color = make_color!(138, 226, 52);
/// Chameleon - Regular - #73D216
pub const GREEN: Color = make_color!(115, 210, 22);
/// Chameleon - Dark - #4E9A06
pub const DARK_GREEN: Color = make_color!(78, 154, 6);

/// Sky Blue - Light - #729FCF
pub const LIGHT_BLUE: Color = make_color!(114, 159, 207);
/// Sky Blue - Regular - #3465A4
pub const BLUE: Color = make_color!(52, 101, 164);
/// Sky Blue - Dark - #204A87
pub const DARK_BLUE: Color = make_color!(32, 74, 135);

/// Plum - Light - #AD7FA8
pub const LIGHT_PURPLE: Color = make_color!(173, 127, 168);
/// Plum - Regular - #75507B
pub const PURPLE: Color = make_color!(117, 80, 123);
/// Plum - Dark - #5C3566
pub const DARK_PURPLE: Color = make_color!(92, 53, 102);

/// Chocolate - Light - #E9B96E
pub const LIGHT_BROWN: Color = make_color!(233, 185, 110);
/// Chocolate - Regular - #C17D11
pub const BROWN: Color = make_color!(193, 125, 17);
/// Chocolate - Dark - #8F5902
pub const DARK_BROWN: Color = make_color!(143, 89, 2);

/// Straight Black.
pub const BLACK: Color = make_color!(0, 0, 0);
/// Straight White.
pub const WHITE: Color = make_color!(255, 255, 255);

/// Alluminium - Light
pub const LIGHT_GRAY: Color = make_color!(238, 238, 236);
/// Alluminium - Regular
pub const GRAY: Color = make_color!(211, 215, 207);
/// Alluminium - Dark
pub const DARK_GRAY: Color = make_color!(186, 189, 182);

/// Aluminium - Light - #EEEEEC
pub const LIGHT_GREY: Color = make_color!(238, 238, 236);
/// Aluminium - Regular - #D3D7CF
pub const GREY: Color = make_color!(211, 215, 207);
/// Aluminium - Dark - #BABDB6
pub const DARK_GREY: Color = make_color!(186, 189, 182);

/// Charcoal - Light - #888A85
pub const LIGHT_CHARCOAL: Color = make_color!(136, 138, 133);
/// Charcoal - Regular - #555753
pub const CHARCOAL: Color = make_color!(85, 87, 83);
/// Charcoal - Dark - #2E3436
pub const DARK_CHARCOAL: Color = make_color!(46, 52, 54);

/// Transparent
pub const TRANSPARENT: Color = make_color!(0, 0, 0, 0);
