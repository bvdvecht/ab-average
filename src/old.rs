// fn tail(&self) -> i32 {
//     self.list[self.left]
// }

// fn len(&self) -> usize {
//     self.right - self.left + 1
// }

// fn extend(&mut self) {
//     self.right += 1;
//     let new_val = self.list[self.right];
//     self.sum += new_val;
//     self.update_avg();
// }

// fn trim_tail(&mut self) -> bool {
//     let mut cut_sum = 0;
//     for i in self.left..self.right {
//         cut_sum += self.list[i];
//         if self.list[i] as f32 > self.avg {
//             self.left = i + 1;
//             self.sum -= cut_sum;
//             self.update_avg();

//             cut_sum = 0;
//         }
//     }
//     false
// }

// fn aborb_range(&mut self, other_range: &Range) {
//     self.right = other_range.right;
//     self.sum += other_range.sum;
//     self.update_avg();
// }

// fn advance_head(&mut self) -> bool {
//     if self.right >= self.list.len() - 1 { return false }

//     let mut front = Range::new(self.list, self.right + 1, self.right + 1);
//     loop {
//         println!("front: {:?}", front);
//         // if front.avg < self.avg {
//         if front.avg < self.avg {
//             // extend current range into front
//             self.aborb_range(&front);
//             return true;
//         }

//         if front.right < self.list.len() -1 {
//             println!("front has higher avg, exending...");
//             front.extend();
//         } else {
//             break;
//         }
//     }
//     false
// }




// fn min_abaverage_smart_old(list: &[i32]) -> (f32, (usize, usize)) {
//     assert!(list.len() >= 2, "List too short.");
//     let mut range = Range::new(list, 0, 1);

//     while range.advance_head() {
//         range.trim_tail();
//     }
//     range.trim_tail();

//     (range.avg, (range.left, range.right))
// }