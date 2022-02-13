#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Clone)]
pub struct AddOperation {
    numbers: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OperationResult {
    result: i32,
}

impl AddOperation {
    pub fn new(numbers: &[i32]) -> Self {
        AddOperation {
            numbers: numbers.to_vec(),
        }
    }

    pub fn to_output(&self) -> OperationResult {
        OperationResult {
            result: self.numbers.iter().sum(),
        }
    }
}

impl OperationResult {
    pub fn get_result(&self) -> i32 {
        self.result
    }
}

#[cfg(test)]
mod tests {
    use super::AddOperation;

    #[test]
    fn two_plus_two_does_not_equal_five() {
        let add_operation = AddOperation::new(&[2, 2]);
        let result = add_operation.to_output().get_result();
        assert_eq!(result, 4);
    }
}
