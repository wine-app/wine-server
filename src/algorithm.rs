// use crate::models::Wine;

// pub fn assemble_M(wines: Vec<Wine>){
//   let num_wines = wines.len();

//   let M = unsafe {
//     let mut M = ndarray::Array2::<f64>::uninitialized((num_wines, 5));
//     wines
//       .into_iter()
//       .enumerate()
//       .for_each(|(col_ind, wine)| {

//         M[(0, col_ind)] = wine.sweetness as f64 / row_magnitude;
//         M[(1, col_ind)] = wine.tannin as f64 / row_magnitude;
//         M[(2, col_ind)] = wine.acid as f64 / row_magnitude;
//         M[(3, col_ind)] = wine.alcohol as f64 / row_magnitude;
//         M[(4, col_ind)] = wine.body as f64 / row_magnitude;
//       });
//     M
//   };
//   println!("{}", A.dot(&M.t()));

// }


// let A = {
//   let row_magnitude: f64 = [
//     new_wine.sweetness as f64,
//     new_wine.tannin as f64,
//     new_wine.acid as f64,
//     new_wine.alcohol as f64,
//     new_wine.body as f64,
//   ]
//   .iter()
//   .sum();

//   ndarray::Array1::<f64>::from(vec![
//     new_wine.sweetness as f64,
//     new_wine.tannin as f64,
//     new_wine.acid as f64,
//     new_wine.alcohol as f64,
//     new_wine.body as f64,
//   ]) / row_magnitude
// };