use cairo_vm::{
    types::program::Program,
    vm::{runners::cairo_runner::CairoRunner, vm_core::VirtualMachine},
};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

#[cfg(feature = "with_mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "with_mimalloc")]
#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

fn parse_program(c: &mut Criterion) {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    c.bench_function("parse program", |b| {
        b.iter(|| {
            _ = Program::from_bytes(black_box(program.as_slice()), black_box(Some("main")))
                .unwrap();
        })
    });
}

fn build_many_runners(c: &mut Criterion) {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    let program = Program::from_bytes(program.as_slice(), Some("main")).unwrap();
    c.bench_function("build runner", |b| {
        b.iter(|| {
            _ = black_box(
                CairoRunner::new(
                    black_box(&program),
                    black_box("starknet_with_keccak"),
                    black_box(false),
                )
                .unwrap(),
            );
        })
    });
}

fn load_program_data(c: &mut Criterion) {
    //Picked the biggest one at the time of writing
    let program = include_bytes!("../cairo_programs/benchmarks/keccak_integration_benchmark.json");
    let program = Program::from_bytes(program.as_slice(), Some("main")).unwrap();
    c.bench_function("initialize", |b| {
        b.iter_batched(
            || {
                (
                    CairoRunner::new(&program, "starknet_with_keccak", false).unwrap(),
                    VirtualMachine::new(false),
                )
            },
            |(mut runner, mut vm)| _ = black_box(runner.initialize(black_box(&mut vm)).unwrap()),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(runner, build_many_runners, load_program_data, parse_program);
criterion_main!(runner);
