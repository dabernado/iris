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
            let test_zeroi = Sum::new(
                1,
                CellPtr::new_with(mem.alloc(1337 as u32).unwrap())
                );

            assert!(test_zeroi.tag() == cast_data.tag());
            assert!(test_zeroi.data()
                    .get(&mem)
                    .as_ref(&mem)
                    == cast_data.data()
                    .get(&mem)
                    .as_ref(&mem)
                   );

            // exec zeroe
            match thread.eval_next_instr(&mem).unwrap() {
                EvalStatus::Pending => {
                    let result = thread.data().get(&mem);
                    let cast_result = unsafe {
                        result.cast::<Nat>(&mem)
                    };
                    let test_result = 1337 as u32;

                    assert!(&test_result == cast_result.as_ref(&mem));
                },
                _ => panic!("eval_next_instr failed"),
            }
        },
        _ => panic!("eval_next_instr failed"),
    }
}
