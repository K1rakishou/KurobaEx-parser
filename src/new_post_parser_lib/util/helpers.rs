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

pub trait MapJoin<T> {
  fn map_join(&self, mapper: &dyn Fn(&T) -> &str) -> String;
  fn map_join_cap(&self, capacity: usize, separator: &str, mapper: &dyn Fn(&T) -> &str) -> String;
}

impl<T> MapJoin<T> for Iter<'_, T> {
  fn map_join(&self, mapper: &dyn Fn(&T) -> &str) -> String {
    return self.map_join_cap(16, "", mapper);
  }

  fn map_join_cap(&self, capacity: usize, separator: &str, mapper: &dyn Fn(&T) -> &str) -> String {
    let mut result_string =  String::with_capacity(capacity);
    let mut index = 0;
    let count = self.len();

    for element in self.as_slice() {
      result_string.push_str(mapper(element));

      if index < count {
        result_string.push_str(separator);
      }

      index += 1;
    }

    return result_string;
  }
}