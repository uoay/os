use std::{env, process, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1] as &str {
        "build" => {
            if args.len() < 3 {
                panic!("missing parameter(s)");
            }
            build(&args[2..=(args.len() - 1)])
        }
        "run" => run(),
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
    if args[0] == "kernel" || args[0] == "hello_world"  {
        handle(&args[0]);
    }
}

fn handle(name: &str) {
    process::Command::new("rust-objcopy")
        .args([
            "--strip-all",
            &format!("{}{}", "target/riscv64gc-unknown-none-elf/debug/", name),
            "-O",
            "binary",
            &format!("{}{}{}", "target/riscv64gc-unknown-none-elf/debug/", name, ".bin"),
        ])
        .status()
        .unwrap();
}

fn run() {
    let mut name = String::from_str("hello_world").unwrap();
    build(&[name]);
    name = String::from_str("kernel").unwrap();
    build(&[name]);

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
