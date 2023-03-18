#[derive(Copy, Clone, Debug)]
pub enum Message {
    Init,
    GameTick,
    KillSnake,
    ResetSnake,
}
