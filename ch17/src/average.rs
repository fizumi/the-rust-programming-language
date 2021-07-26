// カプセル化
// Listing 17-1: An AveragedCollection struct that maintains a list of integers and the average of the items in the collection
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Listing 17-2: Implementations of the public methods add, remove, and average on AveragedCollection
impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> Self {
        let mut ac = AveragedCollection {
            list,
            average: 0.
        };
        ac.update_average();
        ac
    }

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

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// cargo test average
#[test]
fn test() {
    let mut ac = AveragedCollection::new(vec![1,2,3]);
    assert_eq!(ac.average(), 2.);
    let num = ac.remove().unwrap();
    assert_eq!(num, 3);
    assert_eq!(ac.average(), 1.5);
    ac.add(0);
    assert_eq!(ac.average(), 1.0);
}