use std::thread::current;

use crate::stack::Stack;

#[cfg(test)]
mod test_pre_order {
    use super::pre_order;


    #[test]
    fn it_works() {
        let mut input = "A+B*C";
        let mut got = pre_order(input);
        assert_eq!(got, "+A*BC");

        input = "A*B+C";
        got = pre_order(input);
        assert_eq!(got, "+*ABC");
    }

    #[test]
    fn it_works_2() {
        let input = "A*B+C+D+E*F*G+H";
        let got = pre_order(input);
        assert_eq!(got, "++++*ABCD**EFGH");
    }

    #[test]
    fn it_works_3() {
        let input = "(A*B+C)-(D+E)*F*(G+H)-U+K*L*Z";
        let got = pre_order(input);
        assert_eq!(got, "+--+*ABC**+DEF+GHU**KLZ");
    }
}

pub fn pre_order(input: &str) -> String {
    let mut stack: Stack<String> = Stack::new();
    let mut output = String::new();

    for curr in input.chars().rev() {
        if !is_operator(&curr.to_string()) && (curr != ')' && curr != '(') {
            output = curr.to_string() + &output;
        } else {
            if curr == '(' {
                // sacar caracteres de la pila hasta encontrar ')'
                let popped = find_parenthesis(&mut stack);
                // colocarlos en el output
                for i in popped.into_iter() {
                    output = i + &output;
                }
                // seguir con el siguiente caracter
                continue;
            }
            if is_operator(&curr.to_string()) {
                let default_op = "@".to_string();
                let peek = stack.peek().unwrap_or(&default_op);
                if is_gt(peek, &curr.to_string()) {
                    // sacar de la pila hasta encontrar operador igual que actual
                    let popped = find_same_operator(&mut stack, &curr.to_string());
                    // valores retirados son colocados en el output
                    for i in popped.into_iter() {
                        output = i + &output;
                    }
                }
            }
            stack.push(curr.to_string());
        }
        dbg!(&stack.data);
    }

    while let Some(value) = stack.pop() {
        output = value + &output;
    }

    return output;
}

fn is_operator(c: &str) -> bool {
    if c == "+" || c == "*" || c == "-" || c == "/" {
        return true;
    }
    return false;
}

fn is_gt(a: &str, b: &str) -> bool {
    return is_major_order(a) && !is_major_order(b)
}

fn minor_order(a: &str, b: &str) -> bool {
    return !is_major_order(a) && !is_major_order(b)
}

fn is_major_order(a: &str) -> bool {
    return a == "/" || a == "*" 
}

fn find_parenthesis(stack: &mut Stack<String>) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    while let Some(current) = stack.pop() {
        if current == ")" {
           break;
        }
        buffer.push(current);
    }
    return  buffer;
}

fn find_same_operator(stack: &mut Stack<String>, op: &str) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();

    while let Some(current) = stack.peek() {
        if minor_order(&current, op) {
            break;
        }
        let val = stack.pop().unwrap();
        buffer.push(val);
    }

    return buffer;
}
