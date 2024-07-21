impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<i32> = Vec::new();
        
        for (index, temperature) in temperatures.iter().enumerate() {
            let index: i32 = index as i32;
            
            // Process temperatures cooler than the current one
            while !stack.is_empty() && *temperature > temperatures[*stack.last().unwrap() as usize] {
                let previous_index: i32 = stack.pop().unwrap();
                result[previous_index as usize] = index - previous_index;
            }
            
            // Add current index to the stack
            stack.push(index);
        }
        
        result
    }
}
