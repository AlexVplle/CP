struct MinStack {
    stack: Vec<i64>,
    min_value: i64,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack { 
            stack: Vec::new(),
            min_value: i64::MAX,
        }
    }
    
    fn push(&mut self, val: i32) {
        let val_i64: i64 = val as i64;
        if val_i64 < self.min_value {
            self.stack.push(2 * val_i64 - self.min_value);
            self.min_value = val_i64;
        }
        else {
            self.stack.push(val_i64);
        }
    }
    
    fn pop(&mut self) {
        if let Some(last_value) = self.stack.pop() {
            if last_value < self.min_value {
                self.min_value = 2 * self.min_value - last_value;
            }
        }
    }
    
    fn top(&self) -> i32 {
        self.stack.last().map(|last_value: &i64| {
            if *last_value < self.min_value {
                self.min_value as i32
            } 
            else {
                *last_value as i32
            }
        }).unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.min_value as i32
    }
}
