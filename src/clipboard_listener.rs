extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub struct ClipboardListener {
    ctx: ClipboardContext,
    current_clip: String,
}

impl ClipboardListener {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut ctx = ClipboardContext::new()?;
        let current_clip = ctx.get_contents().unwrap_or_default();

        Ok(ClipboardListener { ctx, current_clip })
    }

    pub fn have_new_clip(&mut self) -> bool {
        match self.ctx.get_contents() {
            Ok(clip) => clip != self.current_clip,
            Err(_) => false,
        }
    }

    pub fn get_new_clip(&mut self) -> Option<String> {
        match self.ctx.get_contents() {
            Ok(clip) => {
                self.current_clip = clip.clone();
                Some(clip)
            }
            Err(_) => None,
        }
    }

    pub fn set_clipboard(&mut self, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.ctx.set_contents(contents.to_owned())?;
        Ok(())
    }
}
