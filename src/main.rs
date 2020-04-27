//! A demonstration of constructing and using a non-blocking stream.
//!
//! Audio from the default input device is passed directly to the default output device in a duplex
//! stream, so beware of feedback!

mod synth;
mod operator;

fn main() {
    match synth::run(){
        Ok(_) => {},
        Err(x) => println!("It failed"),
    }
}
