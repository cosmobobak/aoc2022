// use adqselect::nth_element;

pub fn get_task(task_id: usize) -> String {
    std::fs::read_to_string(format!("tasks/task{:02}.txt", task_id)).expect("Error in file fetch.")
}

// pub fn median_element<T: Ord>(s: &mut [T]) -> &mut T {
//     // mutates the input slice in order to find
//     // the median element in O(n) time
//     let idx = s.len() / 2;
//     nth_element(s, idx, &mut Ord::cmp);
//     &mut s[idx]
// }

pub trait CollectIntoVec<T>: Iterator<Item = T> {
    fn vec(self) -> Vec<T>;
}

impl<T, I: Iterator<Item = T>> CollectIntoVec<T> for I {
    fn vec(self) -> Vec<T> {
        self.collect()
    }
}
