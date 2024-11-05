use chia::clvm_traits::{clvm_list, clvm_quote, ToClvm};
use clvmr::{
    chia_dialect::ENABLE_KECCAK, run_program, serde::node_to_bytes, Allocator, ChiaDialect,
};

fn main() -> anyhow::Result<()> {
    let mut a = Allocator::new();

    let keccak = clvm_list!(62, clvm_quote!("hello"));
    let eq = clvm_list!(9, keccak, clvm_quote!("hello"));
    let raise = clvm_list!(8);
    let i = clvm_list!(3, eq, (), raise);
    let program =
        clvm_list!(36, clvm_quote!(142), clvm_quote!(2), clvm_quote!(i), ()).to_clvm(&mut a)?;
    let solution = ().to_clvm(&mut a)?;

    let result = run_program(
        &mut a,
        &ChiaDialect::new(ENABLE_KECCAK),
        program,
        solution,
        u64::MAX,
    )?
    .1;

    println!("{}", hex::encode(node_to_bytes(&a, result)?));

    Ok(())
}
