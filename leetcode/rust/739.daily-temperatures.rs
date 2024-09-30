impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let number_of_temperature: usize = temperatures.len();
        let mut stack: Vec<usize> = Vec::with_capacity(number_of_temperature);
        let mut result: Vec<i32> = vec![0; number_of_temperature];
        temperatures
            .iter()
            .enumerate()
            .for_each(|(index, temperature): (usize, &i32)| {
                while let Some(&top_index) = stack.last() {
                    if temperatures[top_index] < *temperature {
                        result[top_index] = (index - top_index) as i32;
                        stack.pop();
                    } else {
                        break;
                    }
                }
                stack.push(index);
            });
        result
    }
}
