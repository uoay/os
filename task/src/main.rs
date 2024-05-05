use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1] as &str {
        "build" => {
            if args.len() < 3 {
                panic!("missing parameter(s)");
            }
            build(&args[2..=(args.len() - 1)])
        }
        "run" => {
            build(&vec!["kernel".to_string()]);
            run();
        }
        _=>panic!("invalid parameter(s)")
    };
}

fn build(args: &[String]) {
    if args[0] == "task" {
        panic!("cannot build task itself")
    }
    let cur = env::current_dir().unwrap();
    env::set_current_dir(&args[0]).unwrap();
    process::Command::new("cargo")
        .args(["build", "-p", &args[0]])
        .status()
        .unwrap();
    env::set_current_dir(cur).unwrap();
    if args[0] == "kernel" {
        handle_kernel();
    }
}

fn handle_kernel() {
    process::Command::new("rust-objcopy")
        .args([
            "--strip-all",
            "target/riscv64gc-unknown-none-elf/debug/kernel",
            "-O",
            "binary",
            "target/riscv64gc-unknown-none-elf/debug/kernel.bin"
        ])
        .status()
        .unwrap();
}

fn run() {
    process::Command::new("qemu-system-riscv64")
        .args([
            "-machine",
            "virt",
            "-nographic",
            "-bios",
            "default",
            "-kernel",
            "target/riscv64gc-unknown-none-elf/debug/kernel.bin"
        ])
        .status()
        .unwrap();
}
