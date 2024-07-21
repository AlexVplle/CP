impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        tokens.into_iter().for_each(|token: String| {
            match token.as_str() {
                "+" => {
                    let right: i32 = stack.pop().unwrap();
                    let left: i32 = stack.pop().unwrap();
                    stack.push(left + right)
                } 
                "-" => {
                    let right: i32 = stack.pop().unwrap();
                    let left: i32 = stack.pop().unwrap();
                    stack.push(left - right);
                },
                "*" => {
                    let right: i32 = stack.pop().unwrap();
                    let left: i32 = stack.pop().unwrap();
                    stack.push(left * right);
                } ,
                "/" => {
                    let right: i32 = stack.pop().unwrap();
                    let left: i32 = stack.pop().unwrap();
                    stack.push(left / right);
                } ,
                _ => stack.push(token.parse().unwrap()) 
            };
        });
        stack.pop().unwrap()
    }
}
