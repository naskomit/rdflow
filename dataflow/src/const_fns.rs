// #![feature(adt_const_params)]
// #![feature(generic_const_exprs)]

// use dataflow_core::block::BlockSize;
// use const_default::ConstDefault;

// pub const fn f1<const bs: BlockSize>(n: usize) -> usize {
//   // Doesnt work as of yet at least
//   // const m: usize = f2::<{bs}>();
//   // let x: [i32; m];
//   2 * n

// }

// const fn f2<const bs: BlockSize>() -> usize {
//   bs.r_param
// }

// pub fn test_constants() {
//   const x11: usize = 4;
//   const bs: BlockSize = BlockSize::DEFAULT;
//   const bs1: BlockSize = BlockSize {r_param: 3, ..bs};
//   const res1: usize = f1::<bs1>(x11);


//   println!("=========== Const fns ============");
//   println!("{}", res1);

// }