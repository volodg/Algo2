use termion::{color, style};

fn foobar(_x: i32) {
    let color = color::Fg(color::LightCyan);
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
    };
    println!("{}x = {}", color, x);
}

fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    foobar(x); // the type of `x` will be inferred from here

    println!("{}{}Hello, world!", color::Fg(color::LightMagenta), style::Bold);
}
