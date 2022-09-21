use iris::bytecode::*;
use iris::constants::*;
use iris::data::*;
use iris::memory::{Memory, MutatorView};
use iris::op::*;
use iris::safeptr::*;
use iris::vm::{EvalStatus, Thread};

#[test]
fn test_zeroi_zeroe() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push ZEROI and ZEROE onto function
    test_fn.push(&mem, encode_i(OP_ZEROI, 0).unwrap());
    test_fn.push(&mem, encode_i(OP_ZEROE, 0).unwrap());

    // create thread
    let data = mem.alloc(1337 as u32).unwrap();
    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec zeroi
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe { new_data.cast::<Sum<Nat>>(&mem) };

            assert!(1 == cast_data.tag());
            assert!(&1337 == cast_data.data(&mem).as_ref(&mem));

            // exec zeroe
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Nat>(&mem)
                    };

                    assert!(&1337 == cast_result.as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}

#[test]
fn test_uniti_unite() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push UNITI and UNITE onto function
    test_fn.push(&mem, encode_i(OP_UNITI, 0).unwrap());
    test_fn.push(&mem, encode_i(OP_UNITE, 0).unwrap());

    // create thread
    let data = mem.alloc(1337 as u32).unwrap();
    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec uniti
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe {
                new_data.cast::<Product<Unit, Nat>>(&mem)
            };

            assert!(&Unit::new() == cast_data.fst(&mem).as_ref(&mem));
            assert!(&1337 == cast_data.snd(&mem).as_ref(&mem));

            // exec unite
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Nat>(&mem)
                    };

                    assert!(&1337 == cast_result.as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}

#[test]
fn test_swapp() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push two SWAPP instructions onto function
    test_fn.push(&mem, encode_i(OP_SWAPP, 0).unwrap());
    test_fn.push(&mem, encode_i(OP_SWAPP, 0).unwrap());

    // create thread
    let data = mem.alloc(Product::new(
            CellPtr::new_with(mem.alloc(420 as u32).unwrap()),
            CellPtr::new_with(mem.alloc(69 as u32).unwrap()),
            )).unwrap();

    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec swapp
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe {
                new_data.cast::<Product<Nat, Nat>>(&mem)
            };

            assert!(&69 == cast_data.fst(&mem).as_ref(&mem));
            assert!(&420 == cast_data.snd(&mem).as_ref(&mem));

            // exec swapp
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Product<Nat, Nat>>(&mem)
                    };

                    assert!(&420 == cast_result.fst(&mem).as_ref(&mem));
                    assert!(&69 == cast_result.snd(&mem).as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}

#[test]
fn test_assrp_asslp() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push ASSRP + ASSLP instructions onto function
    test_fn.push(&mem, encode_i(OP_ASSRP, 0).unwrap());
    test_fn.push(&mem, encode_i(OP_ASSLP, 0).unwrap());

    // create thread
    let inner = mem.alloc(Product::new(
                    CellPtr::new_with(mem.alloc(420 as u32).unwrap()),
                    CellPtr::new_with(mem.alloc(69 as u32).unwrap()),
            )).unwrap();
    let data = mem.alloc(Product::new(
            CellPtr::new_with(inner),
            CellPtr::new_with(mem.alloc(1337 as u32).unwrap())
    )).unwrap();

    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec assrp
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe {
                new_data.cast::<Product<Nat, Product<Nat, Nat>>>(&mem)
            };
            let inner_data = cast_data.snd(&mem);

            assert!(&420 == cast_data.fst(&mem).as_ref(&mem));
            assert!(&69 == inner_data.fst(&mem).as_ref(&mem));
            assert!(&1337 == inner_data.snd(&mem).as_ref(&mem));

            // exec asslp
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Product<Product<Nat, Nat>, Nat>>(&mem)
                    };
                    let cast_inner = cast_result.fst(&mem);
                    
                    assert!(&420 == cast_inner.fst(&mem).as_ref(&mem));
                    assert!(&69 == cast_inner.snd(&mem).as_ref(&mem));
                    assert!(&1337 == cast_result.snd(&mem).as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}

#[test]
fn test_swaps() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push two SWAPS instructions onto function
    // with type ((nat + nat) + nat) + (nat + nat)
    test_fn.push(&mem, encode_s(OP_SWAPS, 3, 2).unwrap());
    test_fn.push(&mem, encode_s(OP_SWAPS, 2, 3).unwrap());

    // create thread
    let data = mem.alloc(
        Sum::new(2, CellPtr::new_with(mem.alloc(69 as u32).unwrap())))
        .unwrap();
    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec swaps
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe {
                new_data.cast::<Sum<Nat>>(&mem)
            };

            assert!(4 == cast_data.tag());
            assert!(&69 == cast_data.data(&mem).as_ref(&mem));

            // exec swaps
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Sum<Nat>>(&mem)
                    };

                    assert!(2 == cast_data.tag());
                    assert!(&69 == cast_data.data(&mem).as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}

#[test]
fn test_dist_fact() {
    let binding = Memory::new();
    let mem = MutatorView::new(&binding);
    let test_fn = Function::alloc(&mem).unwrap();

    // push DIST + FACT instructions onto function
    // with type ((nat + nat) + nat) * 1,
    // initial div is 1 and second div is 0
    test_fn.push(&mem, encode_s(OP_DIST, 2, 1).unwrap());
    test_fn.push(&mem, encode_s(OP_FACT, 2, 1).unwrap());

    // create thread
    let data = mem.alloc(Product::new(
                    CellPtr::new_with(mem.alloc(
                            Sum::new(2, CellPtr::new_with(
                                    mem.alloc(1337 as u32).unwrap()))
                            ).unwrap()),
                    CellPtr::new_with(mem.alloc(Unit::new()).unwrap()),
            )).unwrap();

    let thread = Thread::alloc_with_arg(
        &mem,
        CellPtr::new_with(data.as_untyped(&mem))
        ).unwrap();

    thread.add_func(&mem, test_fn);
    thread.call_func(&mem, 0, false).unwrap();

    // exec dist
    match thread.eval_next_instr(&mem).unwrap() {
        EvalStatus::Pending => {
            let new_data = thread.data().get(&mem);
            let cast_data = unsafe {
                new_data.cast::<Product<Nat, Product<Nat, Nat>>>(&mem)
            };
            let inner_data = cast_data.snd(&mem);

            assert!(&420 == cast_data.fst(&mem).as_ref(&mem));
            assert!(&69 == inner_data.fst(&mem).as_ref(&mem));
            assert!(&1337 == inner_data.snd(&mem).as_ref(&mem));

            // exec asslp
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Product<Product<Nat, Nat>, Nat>>(&mem)
                    };
                    let cast_inner = cast_result.fst(&mem);
                    
                    assert!(&420 == cast_inner.fst(&mem).as_ref(&mem));
                    assert!(&69 == cast_inner.snd(&mem).as_ref(&mem));
                    assert!(&1337 == cast_result.snd(&mem).as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}
