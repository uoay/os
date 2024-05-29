    .section .data
    .global _num_app
_num_app:
    .quad 1
    .quad _app_start
    .quad _app_end

    .section .data
    .global _app_start
    .global _app_end
_app_start:
    .incbin "./target/riscv64gc-unknown-none-elf/debug/hello_world.bin"
_app_end:
