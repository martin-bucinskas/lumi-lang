use criterion::{criterion_group, Criterion};
use lumi::vm::VM;

fn get_test_vm() -> VM {
    VM::new()
}

fn execute_load(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![0, 0, 100, 0];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_load", move |b| b.iter_with_large_drop(clos));
}

fn execute_load_f64(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![22, 0, 100, 0];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_load_f64", move |b| b.iter_with_large_drop(clos));
}

fn execute_allocate(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![27, 1, 0, 0];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_allocate", move |b| b.iter_with_large_drop(clos));
}

fn execute_lui(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![39, 1, 12, 34];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_lui", move |b| b.iter_with_large_drop(clos));
}

fn execute_load_memory(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![42, 0, 1, 0];
        test_vm.program = bytecode;
        test_vm.heap.resize(10, 0);
        test_vm.run_once();
    };

    c.bench_function("execute_load_memory", move |b| b.iter_with_large_drop(clos));
}

fn execute_set_memory(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![43, 0, 1, 0];
        test_vm.program = bytecode;
        test_vm.heap.resize(10, 0);
        test_vm.run_once();
    };

    c.bench_function("execute_set_memory", move |b| b.iter_with_large_drop(clos));
}

fn execute_push_to_stack(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![44, 0, 0, 0];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_push_to_stack", move |b| {
        b.iter_with_large_drop(clos)
    });
}

fn execute_pop_from_stack(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![45, 0, 0, 0];
        test_vm.program = bytecode;
        test_vm.stack.push(0);
        test_vm.run_once();
    };

    c.bench_function("execute_pop_from_stack", move |b| {
        b.iter_with_large_drop(clos)
    });
}

criterion_group! {
    name = memory;
    config = Criterion::default();
    targets =
    execute_load,
    execute_load_f64,
    execute_allocate,
    execute_lui,
    execute_load_memory,
    execute_set_memory,
    execute_push_to_stack,
    execute_pop_from_stack
}
