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

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack: Vec<String> = new_stack(size_expr);
    let mut post_fix: Vec<String> = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if size(&stack) == 0 {
                    push(&mut stack, i, size_expr)
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expr);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            post_fix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        push(&mut stack, i, size_expr)
                    }
                }
            }

            "(" => push(&mut stack, i, size_expr),
            ")" => {
                while stack.last().unwrap() != "(" {
                    post_fix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }
            _ => post_fix.push(i),
        }
    }

    while size(&stack) != 0 {
        post_fix.push(pop(&mut stack).unwrap());
    }

    println!("{:?}", post_fix);

    post_fix
}

fn priority(x: &String) -> u8 {
    if (x == "+") | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
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
            }else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }      
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

    let postfix = infix_to_postfix(input_expression_tokenized);
}
