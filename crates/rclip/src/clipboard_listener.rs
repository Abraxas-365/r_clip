extern crate clipboard;

use color_eyre::eyre::Result;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub struct ClipboardListener {
    ctx: ClipboardContext,
}

impl ClipboardListener {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let ctx = ClipboardContext::new()?;

        Ok(ClipboardListener { ctx })
    }

    pub fn set_clipboard(&mut self, contents: &str) -> Result<()> {
        let modified_contents = contents.replace('\x00', "\n");
        self.ctx
            .set_contents(modified_contents.to_owned())
            .map_err(|e| color_eyre::eyre::format_err!("Can't set clipboard context: {e}"))
    }
}
