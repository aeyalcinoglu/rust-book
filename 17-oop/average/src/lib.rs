#[derive(Debug, Clone, PartialEq)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

impl Default for AveragedCollection {
    fn default() -> AveragedCollection {
        AveragedCollection {
            list: vec![1, 2, 3],
            average: 2 as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_average_is_two() {
        let default_averaged_col = AveragedCollection::default();
        assert_eq!(default_averaged_col.get_average(), 2 as f64);
    }

    #[test]
    fn add_remove_idempotence() {
        let mut default_averaged_col = AveragedCollection::default();

        let pre_operation_averaged_col = default_averaged_col.clone();

        default_averaged_col.add(1000);
        // use -- --nocapture to see this
        println!(
            "pre: {:?}, post: {:?}",
            pre_operation_averaged_col, default_averaged_col
        );
        default_averaged_col.remove();

        assert_eq!(pre_operation_averaged_col, default_averaged_col);
    }
}
