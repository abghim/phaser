#![allow(unused)]

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);
impl Color {
    fn to_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }
    fn to_ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }

    fn mix(&self, other: &Self, f: f32) -> Color {
        if f > 1.0 || f < 0.0 {
            panic!("Bad mix factor, must be between [0,1]");
        }

        Color(
            ((self.0 as f32) * (1.0 - f) + (other.0 as f32) * f) as u8,
            ((self.1 as f32) * (1.0 - f) + (other.1 as f32) * f) as u8,
            ((self.2 as f32) * (1.0 - f) + (other.2 as f32) * f) as u8,
        )
    }

    fn hex_str(&self) -> String {
        format!("#{}{}{}", format_radix(self.0 as u32, 16), format_radix(self.1 as u32, 16), format_radix(self.2 as u32, 16))
    }
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}


fn cprint(c: &str, c1: Color, c2: Color) {
    print!("{}{}{}\x1b[0m", c1.to_ansi(), c2.to_ansi_bg(), c);
}

fn tmux_print(c: &str, c1: Color, c2: Color) {
    print!("#[fg={},bg={}]{}#[fg=default,bg=default]", c1.hex_str(), c2.hex_str(), c);
}



fn hex_to_color(hx: u32) -> Color {
    Color(
        ((hx >> 16) & 0xff) as u8,
        ((hx >> 8) & 0xff) as u8,
        (hx & 0xff) as u8,
    )
}


const GLYPH: &str = "";

fn main() {
    if std::env::args().len() != 5 {
        println!("Usage: phaser [color1-hex] [color2-hex] [len] [message]");
        return;
    }
    let mut args = std::env::args().skip(1);
    let c1 = hex_to_color(u32::from_str_radix(args.next().unwrap().as_str(), 16).unwrap_or(0));
    let c2 = hex_to_color(u32::from_str_radix(args.next().unwrap().as_str(), 16).unwrap_or(0));
    let n = (u32::from_str_radix(args.next().unwrap().as_str(), 10).unwrap());
    let msg = args.next().unwrap();

    // cprint(&msg, Color(255, 255, 255), c1);
    cprint(&msg, Color(255, 255, 255), c1);

    let step: f32 = 1.0/(n as f32);

    for i in 1..(n+1) {
        let fnow = (i as f32)*step;
        let fprev = (i as f32 -1.0)*step;

        let cnow = c1.mix(&c2, fnow);
        let cprev = c1.mix(&c2, fprev);

        // cprint(GLYPH, cprev, cnow);
        cprint(GLYPH, cprev, cnow);
        
    }
}
