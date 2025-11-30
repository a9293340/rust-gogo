mod chapter10;
mod chapter10_2;
mod chapter10_3;
mod chapter2;
mod chapter3;
mod chapter5;
mod chapter6;

// !windows 屏蔽 warning : $env:RUSTFLAGS="-Awarnings" cargo r
// !mac 屏蔽 warning : RUSTFLAGS="-Awarnings" cargo run

pub fn main() {
    // chapter2::main();
    // chapter3::main();
    // chapter5::main();
    // chapter10::main();
    // chapter10_2::main();
    chapter10_3::main();
}
