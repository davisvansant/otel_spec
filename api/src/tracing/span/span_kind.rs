#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SpanKind {
    Server,
    Client,
    Producer,
    Consumer,
    Internal,
}
