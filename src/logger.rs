pub trait Logger {
    fn log(&mut self, value: String);
}

#[derive(Default)]
pub struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&mut self, value: String) {
        println!("{}", value);
    }
}

#[derive(Default)]
pub struct TestLogger {
    pub stack: Vec<String>,
}
impl Logger for TestLogger {
    fn log(&mut self, value: String) {
        self.stack.push(value);
    }
}
