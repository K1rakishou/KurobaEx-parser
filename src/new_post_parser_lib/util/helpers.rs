use std::slice::Iter;

pub trait SumBy<T> {
  fn sum_by(&self, func: &dyn Fn(&T) -> i32) -> i32;
}

impl<T> SumBy<T> for Iter<'_, T> {
  fn sum_by(&self, func: &dyn Fn(&T) -> i32) -> i32 {
    let mut sum: i32 = 0;

    for element in self.as_slice() {
      sum += func(&element);
    }

    return sum;
  }
}

pub trait LastIndex<T> {
  fn last_index(&self) -> Option<usize>;
}

impl<T> LastIndex<T> for Iter<'_, T> {
  fn last_index(&self) -> Option<usize> {
    if self.len() == 0 {
      return Option::None;
    }

    return Option::Some(self.len() - 1);
  }
}