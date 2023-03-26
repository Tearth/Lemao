use lemao_core::lemao_common_platform::input::InputEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Init,
    GameTick,
    KillSnake,
    ResetSnake,
    FoodEaten,
    InputEvent(InputEvent),
}
