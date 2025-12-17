use art12134::PrimaryColor;
use art12134::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    let orange = mix(red, yellow);
}