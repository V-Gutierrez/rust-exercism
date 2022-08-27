#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/*
If there is not exactly one element in the stack at the end, return None.

If there is an operator with too few operands (such as the input 2 +), return None.
*/

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut ram_stack: Vec<i32> = vec![];

    if inputs.is_empty() {
        return None;
    }

    for item in inputs {
        match item {
            CalculatorInput::Value(x) => ram_stack.push(*x),
            _ => {
                if ram_stack.len() < 2 {
                    return None;
                }

                let b = ram_stack.pop()?;
                let a = ram_stack.pop()?;

                match item {
                    CalculatorInput::Add => ram_stack.push(a + b),
                    CalculatorInput::Multiply => ram_stack.push(a * b),
                    CalculatorInput::Subtract => ram_stack.push(a - b),
                    CalculatorInput::Divide => ram_stack.push(a / b),
                    _ => {
                        return None;
                    }
                }
            }
        }
    }

    if ram_stack.len() != 1 {
        return None;
    } else {
        return Some(ram_stack[0]);
    }
}