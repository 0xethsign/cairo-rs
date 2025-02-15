use iai::{black_box, main};

use cairo_vm::{
    types::program::Program,
    vm::{runners::cairo_runner::CairoRunner, vm_core::VirtualMachine},
};

#[cfg(feature = "with_mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "with_mimalloc")]
#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

fn parse_program() {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    let program =
        Program::from_bytes(black_box(program.as_slice()), black_box(Some("main"))).unwrap();
    let _ = black_box(program);
}

fn build_many_runners() {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    let program = Program::from_bytes(program.as_slice(), Some("main")).unwrap();
    for _ in 0..100 {
        let runner = CairoRunner::new(black_box(&program), "starknet_with_keccak", false).unwrap();
        let _ = black_box(runner);
    }
}

fn load_program_data() {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    let program = Program::from_bytes(program.as_slice(), Some("main")).unwrap();
    for _ in 0..100 {
        let mut runner = CairoRunner::new(&program, "starknet_with_keccak", false).unwrap();
        let mut vm = VirtualMachine::new(false);
        _ = black_box(runner.initialize(black_box(&mut vm)).unwrap());
    }
}

main!(parse_program, build_many_runners, load_program_data);
