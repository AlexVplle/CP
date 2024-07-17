struct MinStack {
    stack: Vec<i32>,
    min_values: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack { 
            stack: Vec::new(),
            min_values: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_values.last().map_or(true, |min: &i32| *min >= val ) {
            self.min_values.push(val);
        }
    }
    
    fn pop(&mut self) {
        if let Some(last_value) = self.stack.pop() {
            if Some(&last_value) == self.min_values.last() {
                self.min_values.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_values.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
