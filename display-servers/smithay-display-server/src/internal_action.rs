use leftwm_core::{DisplayAction, Window};

use crate::SmithayWindowHandle;

#[derive(Debug)]
pub enum InternalAction {
    Flush,
    GenerateVerifyFocusEvent,
    UpdateWindows(Vec<Window<SmithayWindowHandle>>),
    DisplayAction(DisplayAction<SmithayWindowHandle>),
}
