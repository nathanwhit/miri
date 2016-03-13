#![feature(custom_attribute)]
#![allow(dead_code, unused_attributes)]

// #[miri_run(expected = "Int(1)")]
// fn one_line_ref() -> i32 {
//     *&1
// }

// #[miri_run(expected = "Int(1)")]
// fn basic_ref() -> i32 {
//     let x = &1;
//     *x
// }

// #[miri_run(expected = "Int(3)")]
// fn basic_ref_mut() -> i32 {
//     let x = &mut 1;
//     *x += 2;
//     *x
// }

// #[miri_run(expected = "Int(3)")]
// fn basic_ref_mut_var() -> i32 {
//     let mut a = 1;
//     {
//         let x = &mut a;
//         *x += 2;
//     }
//     a
// }