fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop (stack: &mut Vec<String>) -> Option<String> {
    let poped_value = stack.pop();
    poped_value
}

fn push (stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    }
    else {
        stack.push(item);
    }
}

fn size (stack: &Vec<String>) -> usize {
    stack.len()
}

fn main() {
   
}
