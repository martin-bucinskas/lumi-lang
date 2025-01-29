use criterion::{criterion_group, Criterion};
use lumi::vm::VM;

fn get_test_vm() -> VM {
    VM::new()
}

fn execute_add(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![1, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_add", move |b| b.iter(clos));
}

fn execute_sub(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![2, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_sub", move |b| b.iter(clos));
}

fn execute_mul(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![3, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_mul", move |b| b.iter(clos));
}

fn execute_div(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![4, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 2;
        test_vm.run_once();
    };

    c.bench_function("execute_div", move |b| b.iter(clos));
}

fn execute_add_f64(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![23, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_add_f64", move |b| b.iter(clos));
}

fn execute_sub_f64(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![24, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_sub_f64", move |b| b.iter(clos));
}

fn execute_mul_f64(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![25, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.run_once();
    };

    c.bench_function("execute_mul_f64", move |b| b.iter(clos));
}

fn execute_div_f64(c: &mut Criterion) {
    let clos = || {
        let mut test_vm = get_test_vm();
        let bytecode = vec![26, 0, 1, 2];
        test_vm.program = bytecode;
        test_vm.float_registers[0] = 10.123;
        test_vm.float_registers[1] = 2.123;
        test_vm.run_once();
    };

    c.bench_function("execute_div_f64", move |b| b.iter(clos));
}

criterion_group! {
    name = arithmetic;
    config = Criterion::default();
    targets =
    execute_add,
    execute_sub,
    execute_mul,
    execute_div,
    execute_add_f64,
    execute_sub_f64,
    execute_mul_f64,
    execute_div_f64
}
