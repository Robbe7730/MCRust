pub enum ErrorType {
    Recoverable(String),
    Fatal(String),
    GracefulExit,
}
