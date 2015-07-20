use screen;

#[cfg(not(test))]
pub struct Console {
  rect: screen::Rect,
}

#[cfg(not(test))]
impl Console {
  pub fn new() -> Console {
    Console {
      rect: screen::Rect(screen::Cell(0,0), screen::Size(1, 0))
    }
  }

  pub fn update_size(&mut self, row: u16, cols: u16) {
    self.rect = screen::Rect(screen::Cell(row,0), screen::Size(1, cols))
  }

  pub fn print(&self, message: &str, screen: &mut screen::Screen) {
    for (x, y) in screen::CellIterator::new(self.rect).zip(message.chars()) {
      screen.put(x, y, screen::Color::Black, screen::Color::White);
    }
  }
}
