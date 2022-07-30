pub mod vm;
pub mod repl;
pub mod assembler;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}

// https://blog.subnetzero.io/post/building-language-vm-part-09/