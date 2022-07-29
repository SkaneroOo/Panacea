pub mod vm;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}

// https://blog.subnetzero.io/post/building-language-vm-part-08/