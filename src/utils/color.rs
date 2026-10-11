use std::io::{self, IsTerminal};

pub struct Style {
   enabled: bool,
}

impl Style {
   pub fn stdout() -> Self {
      Self {
         enabled: io::stdout().is_terminal(),
      }
   }

   pub fn stderr() -> Self {
      Self {
         enabled: io::stderr().is_terminal(),
      }
   }

   pub fn ansi(&self, code: &str, text: &str) -> String {
      if self.enabled {
         format!("\x1b[{code}m{text}\x1b[0m")
      } else {
         text.to_string()
      }
   }

   pub fn rgb(&self, (r, g, b): (u8, u8, u8), text: &str) -> String {
      if self.enabled {
         format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m")
      } else {
         text.to_string()
      }
   }
}
