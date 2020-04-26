use iced::{
  Align, Column, Container, Element, Length, Text
};
use super::Message;

use std::time::{Duration, Instant};

// 引数の文字列をアプリケーションの画面に表示
pub fn show_display_val(dvalue: &String, duration: &Duration) -> Element<'static, Message> {
  const MINUTE: u64 = 60;
  const HOUR: u64 = 60 * MINUTE;
  let seconds = duration.as_secs();
  let duration = Text::new(format!(
    "{:0>2}:{:0>2}:{:0>2}",
    seconds / HOUR,
    (seconds % HOUR) / MINUTE,
    seconds % MINUTE
));
  let content = Column::new()
      .padding(20)
      .spacing(20)
      .max_width(500)
      .align_items(Align::Start)
      .push(Text::new("test:"))
      .push(duration)
      .push(Text::new(dvalue));
  Container::new(content)
      .width(Length::FillPortion(2))
      .height(Length::Fill)
      .into()
}

// メッセージ読み込み中の表示 => Utilにしたい
pub fn loading_message() -> Element<'static, Message> {
Container::new(
    Text::new("Loading...")
        .size(50),
)
.width(Length::Fill)
.height(Length::Fill)
.center_y()
.into()
}