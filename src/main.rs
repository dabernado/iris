use iris::bytecode::*;
use iris::constants::*;
use iris::data::*;
use iris::error::*;
use iris::memory::*;
use iris::op::*;
use iris::safeptr::*;
use iris::vm::*;

fn main() {
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
                new_data.cast::<Sum<Product<Nat, Unit>>>(&mem)
            };
            let inner_data = cast_data.data(&mem);

            assert!(1 == cast_data.tag());
            assert!(&1337 == inner_data.fst(&mem).as_ref(&mem));
            assert!(&Unit::new() == inner_data.snd(&mem).as_ref(&mem));

            // exec fact
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        new_data.cast::<Product<Sum<Nat>, Unit>>(&mem)
                    };
                    let result_sum = cast_result.fst(&mem);

                    println!("expected 2, got: {}", result_sum.tag());
                    //assert!(2 == result_sum.tag());
                    assert!(&1337 == result_sum.data(&mem).as_ref(&mem));
                    assert!(&Unit::new() == cast_result.snd(&mem).as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}
