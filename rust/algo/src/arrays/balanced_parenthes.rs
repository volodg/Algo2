
//https://tproger.ru/problems/algorithm-that-displays-all-valid-combinations-of-pairs-of-brackets/
#[derive(Debug)]
pub struct Solution {
    data: Vec<bool>,
    state: i16,
    capacity: i16
}

impl Solution {
    fn initial(capacity: i16) -> Solution {
        Solution {
            data: vec![],
            state: 0,
            capacity
        }
    }

    fn next(&self) -> Vec<Solution> {
        assert!(!self.is_final());

        let close_bracket = (|| {
            let mut new_data = self.data.clone();
            new_data.push(false);
            Solution {
                data: new_data,
                state: self.state - 1,
                capacity:  self.capacity - 1
            }
        })();

        if self.state == self.capacity {
            return vec![close_bracket];
        }

        let mut result: Vec<Solution> = vec![];

        if self.state != 0 {
            result.push(close_bracket)
        }

        let open_bracket = (|| {
            let mut new_data = self.data.clone();
            new_data.push(true);
            Solution {
                data: new_data,
                state: self.state + 1,
                capacity:  self.capacity
            }
        })();

        result.push(open_bracket);

        result
    }

    fn is_final(&self) -> bool {
        self.capacity == 0
    }

    pub fn build_all(capacity: i16) -> Vec<String> {
        let mut final_solutions: Vec<Solution> = vec![];
        let mut queue: Vec<Solution> = vec![Solution::initial(capacity)];

        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            let next_elements = current.next();
            for el in next_elements {
                if el.is_final() {
                    final_solutions.push(el)
                } else {
                    queue.push(el)
                }
            }
        }

        final_solutions
            .iter()
            .map(|x| x.data.iter().map(|x| if *x { "(" } else { ")" } ).collect() )
            .collect()
    }
}
