mod arithmetic;
mod boolean;
mod list;

pub use arithmetic::*;
pub use boolean::*;
pub use list::*;

use super::callable::Arguments;
use super::Callable;
use crate::error::{Error, ErrorKind};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::{Atom, Node};
use crate::Result;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug)]
struct PrintFunction;

impl PrintFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Callable for PrintFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args =
            PrintFunction::ARGUMENTS.evaluate(intp, arguments_nodes)?;

        writeln!(intp.output, "{}", &evaluated_args[0])?;

        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "print"
    }
}

#[derive(Debug)]
struct ReadFunction;

impl ReadFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Callable for ReadFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_arg = ReadFunction::ARGUMENTS
            .evaluate(intp, argument_nodes)?
            .pop()
            .unwrap();

        if let Value::String(prompt) = evaluated_arg {
            print!("{prompt}");
            std::io::stdout().flush()?;

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;

            Ok(Value::String(buffer.trim_end().to_owned()))
        } else {
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "String",
                values: vec![evaluated_arg],
            }))
        }
    }

    fn name(&self) -> &str {
        "read"
    }
}

#[derive(Debug)]
struct EvalFunction;

impl EvalFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Callable for EvalFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args =
            EvalFunction::ARGUMENTS.evaluate(intp, argument_nodes)?;

        let value = &evaluated_args[0];

        if let Value::String(source) = value {
            let mut intp = Interpreter::with_parser_no(intp.parser_no);

            intp.parser_no += 1;

            intp.interpret(source)
        } else {
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "String",
                values: evaluated_args,
            }))
        }
    }

    fn name(&self) -> &str {
        "eval"
    }
}

#[derive(Debug)]
struct QuoteFunction;

impl QuoteFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Callable for QuoteFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        QuoteFunction::ARGUMENTS.check_amount(argument_nodes.len())?;

        let argument = &argument_nodes[0];

        match argument {
            Node::Atom(Atom::Symbol(symbol)) => {
                Ok(Value::Symbol(symbol.lexeme().to_owned()))
            }
            Node::List(list) => {
                let values = list
                    .expressions
                    .iter()
                    .map(|node| node.accept(intp))
                    .collect::<Result<Vec<_>>>()?;

                Ok(Value::List(values))
            }
            _ => {
                let mut values =
                    QuoteFunction::ARGUMENTS.evaluate(intp, argument_nodes)?;

                Ok(values.pop().unwrap())
            }
        }
    }

    fn name(&self) -> &str {
        "quote"
    }
}

pub fn get_builtin_functions() -> Vec<Rc<dyn Callable>> {
    vec![
        Rc::new(ArithmeticFunction::new(ArithmeticType::Add, 2)),
        Rc::new(ArithmeticFunction::new(ArithmeticType::Subtract, 2)),
        Rc::new(ArithmeticFunction::new(ArithmeticType::Multiply, 2)),
        Rc::new(ArithmeticFunction::new(ArithmeticType::Divide, 2)),
        Rc::new(BooleanFunction::new(BooleanOp::Greater, 2)),
        Rc::new(BooleanFunction::new(BooleanOp::GreaterOrEqual, 2)),
        Rc::new(BooleanFunction::new(BooleanOp::Equal, 2)),
        Rc::new(BooleanFunction::new(BooleanOp::LessOrEqual, 2)),
        Rc::new(BooleanFunction::new(BooleanOp::Less, 2)),
        Rc::new(TailFunction),
        Rc::new(QuoteFunction),
        Rc::new(PrintFunction),
        Rc::new(ReadFunction),
        Rc::new(EvalFunction),
        Rc::new(Increment),
    ]
}