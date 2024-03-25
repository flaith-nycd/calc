use regex::Regex;
use std::collections::HashMap;
use std::io;
// use std::io::Result;

// https://rosettacode.org/wiki/Parsing/RPN_to_infix_conversion
fn get_precedence(operator: &str) -> i32 {
    let precedences = 
        HashMap::from([
            ("+", 1), 
            ("-", 1), 
            ("*", 2), 
            ("/", 2), 
            ("^", 3)
        ]);

    let op_value = precedences.get(operator);

    // As we got an 'Option' returned, use 'match' with 'Some' and 'None'
    match op_value {
        Some(x) => *x, // Something has been found, so return it
        None => 0,     // If not found
    }
}

fn tokenize(expression: &str) -> Vec<&str> {
    /*
       [0-9]*[.]?[0-9]+  : Float value
                      +  : Add
                      -  : Substract
                      *  : Multiply
                      /  : Divide
                      ^  : Power
    */
    let re = Regex::new(r"[0-9]*[.]?[0-9]+|\+|-|\*|/|\(|\)|\^").unwrap();

    let tokens: Vec<&str> = re
        .find_iter(expression)
        .map(|c| c.as_str())
        .collect();

    tokens
}

// https://fr.wikipedia.org/wiki/Algorithme_Shunting-yard
// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn infix_to_postfix(expression: &str) -> Vec<&str> {
    let mut output: Vec<&str> = vec![]; // Output stack
    let mut op_stack: Vec<&str> = vec![]; // Operators stack

    let tokens = tokenize(expression);
    
    println!("TOKENS: {:?}", &tokens);

    for token in tokens {
        // if token is numeric add it to the output
        if let Ok(_num) = token.trim().parse::<f64>() {
            output.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            while !op_stack.is_empty() && *op_stack.last().unwrap() != "(" {
                output.push(op_stack.pop().unwrap());
            }
            op_stack.pop(); // Pop the left parenthesis
        } else {
            while !op_stack.is_empty()
                && get_precedence(*op_stack.last().unwrap()) >= get_precedence(token)
            {
                output.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    while !op_stack.is_empty() {
        output.push(op_stack.pop().unwrap())
    }

    output
}

// No need to return a String (for the error), we used the regex before.
fn evaluate_postfix(expression: Vec<&str>) -> Result<f64, ()> {
    let mut stack: Vec<f64> = vec![];

    for token in expression {
        // if token is a digit (NOT NUMERIC)
        if let Ok(num) = token.trim().parse::<f64>() {
            stack.push(num);
        } else {
            let b = stack.pop().unwrap_or(0.0);
            let a = stack.pop().unwrap_or(0.0);
            match token {
                "+" => stack.push(a + b),
                "-" => stack.push(a - b),
                "*" => stack.push(a * b),
                "/" => stack.push(a / b),
                "^" => stack.push(f64::powf(a, b)),
                _ => (),    // No need to return an error as we used the regex before.
            }
        }
    }

    Ok(stack.pop().unwrap_or(0.0))
}

fn main() {
    println!("Enter infix expression (ex: -->3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3<--)");
    let mut infix_expression = String::new();

    io::stdin()
        .read_line(&mut infix_expression)
        .expect("Failed to read expression");

    let postfix_expression = infix_to_postfix(infix_expression.as_str());

    print!("Postfix expression: ");
    for each_token in &postfix_expression {
        print!("{} ", each_token);
    }
    println!();

    if let Ok(result) = evaluate_postfix(postfix_expression) {
        println!("Result: {}", result);
    }
}
