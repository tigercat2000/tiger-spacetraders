#[derive(Clone, Debug)]
pub struct AsyncMessage {
    pub typ: AsyncMessageType,
}

#[derive(Clone, Debug)]
pub enum AsyncMessageType {
    Register(String),
}
