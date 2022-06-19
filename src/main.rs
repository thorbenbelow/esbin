mod AST;
mod Interpreter;
mod Object;
mod Value;

fn main() {
    let program = AST::Program::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        unimplemented!();
    }
}
