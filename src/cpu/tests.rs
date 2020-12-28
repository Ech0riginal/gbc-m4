use super::{CPU, Instruction, Register};

/* well, if we could test that would be nice, TODO this later

   Compiling gbc-m4 v0.1.0
error[E0463]: can't find crate for `test`
  --> src/cpu/tests.rs:22:1
   |
47 | / fn test_add_8() {
48 | |     let mut cpu = cpu();
49 | |
50 | |     unsafe {
...  |
56 | |     assert_eq!(cpu.a, 9);
57 | | }
   | |_^ can't find crate
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `gbc-m4`

To learn more, run the command again with --verbose.

 */

fn cpu() -> CPU {
    CPU {
        a: 0,
        flag: 0b0000_0000,
        b: 1,
        c: 2,
        d: 3,
        e: 4,
        h: 5,
        l: 6,
        pc: 0,
        sp: 0,
        bus: [0u8; 65535]
    }
}


#[test]
fn test_add_8() {
    let mut cpu = cpu();

    unsafe {
        for _i in 0..2 {
            cpu.execute(Instruction::ADD(Register::D));
        }
    }

    assert_eq!(cpu.a, 9);
}