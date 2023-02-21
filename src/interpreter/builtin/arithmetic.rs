use crate::error::{Error, ErrorKind};
use crate::interpreter::callable::{Arguments, Callable};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Node;
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub enum ArithmeticType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct ArithmeticFunction {
    atype: ArithmeticType,
    arguments: Arguments,
}

impl ArithmeticFunction {
    pub fn new(atype: ArithmeticType, arguments: usize) -> Self {
        Self {
            atype,
            arguments: Arguments::Minimum(arguments),
        }
    }

    pub fn op(&self, left: &Value, right: &Value) -> Result<Value> {
        match self.atype {
            ArithmeticType::Add => left.clone() + right.clone(),
            ArithmeticType::Subtract => left.clone() - right.clone(),
            ArithmeticType::Multiply => left.clone() * right.clone(),
            ArithmeticType::Divide => left.clone() / right.clone(),
        }
    }
}

impl Callable for ArithmeticFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args = self.arguments.evaluate(intp, argument_nodes)?;

        let mut iter = evaluated_args.iter();

        let mut acc = iter.next().expect("More than two elements.").clone();

        for value in iter {
            acc = self.op(&acc, value)?;
        }

        Ok(acc)
    }

    fn name(&self) -> &str {
        match self.atype {
            ArithmeticType::Add => "+",
            ArithmeticType::Subtract => "-",
            ArithmeticType::Multiply => "*",
            ArithmeticType::Divide => "/",
        }
    }
}

#[derive(Debug)]
pub struct Increment;

impl Increment {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Callable for Increment {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args =
            Increment::ARGUMENTS.evaluate(intp, argument_nodes)?;

        let key = &evaluated_args[0];

        if let Value::Symbol(key) = key {
            if let Some(Value::Number(value)) = intp.environment.get_mut(key) {
                *value += 1.0;
                Ok(Value::Nil)
            } else {
                Err(Error::without_location(ErrorKind::InvalidArguments {
                    expected: "Number",
                    values: evaluated_args,
                }))
            }
        } else {
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "String",
                values: evaluated_args,
            }))
        }
    }

    fn name(&self) -> &str {
        "inc"
    }
}