// trait Default {
//     fn default() -> Self;
// }
// struct Color(u8, u8, u8);

// impl Color {
//     fn default() -> Self {
//         Color(0, 0, 0)
//     }
// }

// impl Default for Color {
//     // 默认颜色是黑色 (0, 0, 0)
//     fn default() -> Self {
//         Color{
//             r:1,
//             g:1,
//             b:1,
//         }
//     }
// }

#[derive(Default)]

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Color { r, g, b }
    }
}

impl Color {
    fn red(r: u32) -> Self {
        Color {
            r,
            ..Color::default() // 注意这一句
        }
    }

    fn green(g: u32) -> Self {
        Color {
            g,
            ..Color::default() // 注意这一句
        }
    }

    fn blue(b: u32) -> Self {
        Color {
            b,
            ..Color::default() // 注意这一句
        }
    }
}

#[test]

fn main() {
    let color = Color::default();

    // 或
    let color: Color = Default::default();
}
