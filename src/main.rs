fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let poped_value = stack.pop();
    poped_value
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn individual_symbols(input_exp: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_char: Vec<char> = input_exp.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_char {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            }
            tokenized_input.push(temp.into_iter().collect());
            tokenized_input.push(i.to_string());
            temp = vec![];
        }
    }

    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }

    println!("{:?}", tokenized_input);
    tokenized_input
}

fn main() {
    let input = String::from("(33+45/3*(2+9)-50)");
    println!("The original expression is {:?}", input);
    let input_expression_tokenized = individual_symbols(input);
}
