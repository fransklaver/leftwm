use leftwm_core::{DisplayAction, Window};

use crate::leftwm_config::LeftwmConfig;
use crate::SmithayWindowHandle;

#[derive(Debug)]
pub enum InternalAction {
    Flush,
    GenerateVerifyFocusEvent,
    UpdateConfig(LeftwmConfig),
    UpdateWindows(Vec<Window<SmithayWindowHandle>>),
    DisplayAction(DisplayAction<SmithayWindowHandle>),
}
