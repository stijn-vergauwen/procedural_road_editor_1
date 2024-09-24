#[derive(Clone, PartialEq, Debug)]
pub struct ChangedValue<T> {
    pub previous_value: T,
    pub new_value: T,
}

impl<T> ChangedValue<T> {
    pub fn new(previous_value: T, new_value: T) -> Self {
        Self {
            previous_value,
            new_value,
        }
    }
}
