use std::f64::NAN;

fn main() {
    // objects contain data and behavior
    // encapsulation hides implementation details

    let mut ac = AveragedCollection::new();
    println!("{:?}", ac);

    ac.add(3);
    println!("{:?}", ac);

    ac.add(5);
    println!("{:?}", ac);

    println!("remove {:?}", ac.remove());
    println!("{:?}", ac);

    println!("remove {:?}", ac.remove());
    println!("{:?}", ac);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: NAN,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(_) => {
                self.update_average();
                result
            }
            None => result,
        }
    }
}
