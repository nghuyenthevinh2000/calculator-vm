use calculator_ast_parser::{Result, Compile, Node, Operator, Sign};
use inkwell::{context::Context, types::FloatType, builder::Builder, values::{FloatValue, AnyValue}, execution_engine::JitFunction};

pub struct Compiler;

type CompileFunc = unsafe extern "C" fn() -> f64;

impl Compile for Compiler {
    type Output = Result<f64>;

    // implement fn from_ast()
    fn from_ast(ast: Vec<calculator_ast_parser::Node>) -> Self::Output {
        // llvm context? he LLVMContext is a central component in the LLVM 
        // infrastructure and serves as a container for various global data and settings 
        // that are used during the compilation process.
        let context = Context::create();
        // what is llvm module? The Module class in LLVM represents a single translation 
        // unit or a compilation module in LLVM IR (Intermediate Representation). 
        // This class is part of the LLVM IR library and is used to represent 
        // the entire program being compiled.
        //
        // A translation unit is the LLVM IR representation of a source file. 
        // A program is composed of many translation unit linked together.
        // 
        // In the following code, I am declaring a group of translation units.
        let module = context.create_module("compiler");


        let builder = context.create_builder();

        // declare execution engine
        let execution_engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();
        
        // declare function signature
        let decimal_type = context.f64_type();
        let fn_type = decimal_type.fn_type(&[], false);

        let function = module.add_function("compile", fn_type, None);
        // what is basic block in LLVM?
        // a function is divided into basic blocks, the flow of a function 
        // will go from one block to another until it reaches the end block.
        // it is crucial to understand LLVM block correctly.
        // for example, 
        // define i32 @example_function(i32 %a, i32 %b) {
        //     entry:
        //       %sum = add i32 %a, %b
        //       br label %exit
          
        //     exit:
        //       %result = phi i32 [ %sum, %entry ]
        //       ret i32 %result
        // }
        // instructions? instructions define what needs to be done in a block
        // however, instructions and building blocks relation can be flexible.
        // And, instructions can be inserted later into building blocks or arranged flexibly in different building blocks.
        // Just need to make sure that the function will be executed in correct order.

        let basic_block = context.append_basic_block(function, "entry");
        // setting the builder position to insert instructions into basic blocks
        builder.position_at_end(basic_block);
        
        // recursively add instructions into basic block by traversing ast tree
        for node in ast {
            let recursive_builder = RecursiveBuilder::new(decimal_type);
            let return_value = recursive_builder.build(&node);
            let _ = builder.build_return(Some(&return_value));
        }
        println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );

        // execute function
        unsafe {
            let compile_func: JitFunction<CompileFunc> = execution_engine.get_function("compile").unwrap();

            Ok(compile_func.call())
        }
        

    }
}

struct RecursiveBuilder<'a> {
    f64_type: FloatType<'a>,
}

impl <'a> RecursiveBuilder<'a> {
    fn new(f64_type: FloatType<'a>) -> Self {
        Self{f64_type}
    }

    // ast is currently in the form of

    fn build(&self, ast: &Node) -> FloatValue {
        match ast {
            Node::Number(dec) => self.f64_type.const_float_from_string(&dec.to_string()),
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_num = self.build(lhs);
                let rhs_num = self.build(rhs);

                // perform computation
                match op {
                    Operator::Minus => lhs_num.const_sub(rhs_num),
                    Operator::Plus => lhs_num.const_add(rhs_num),
                    Operator::Mul => lhs_num.const_mul(rhs_num),
                    Operator::Div => lhs_num.const_div(rhs_num)
                }
            }
            Node::UnaryExpr { op, child } => {
                let child = self.build(child);

                match op {
                    Sign::Positive => child,
                    Sign::Negative => child.const_neg()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Compiler::from_source("1 + 2").unwrap(), 3.0);
        assert_eq!(Compiler::from_source("2.5 + (2.1 - 1.8)").unwrap(), 2.8);
        assert_eq!(Compiler::from_source("(2.6 + 3.9) - 1.2").unwrap(), 5.3);
        assert_eq!(Compiler::from_source("1.7 + ((2.3 + 3.1) - (2.9 + 3.5))").unwrap(), 0.7);
        assert_eq!(Compiler::from_source("1.2 * (1.9 + 2.9)").unwrap(), 5.76);
        assert_eq!(Compiler::from_source("(7.8+2.4)/(1.3+2.5)").unwrap(), 2.6842105263157894);
        // parser fails
        // assert_eq!(Jit::from_source("2 + 3 - 1").unwrap(), 4);
    }
}