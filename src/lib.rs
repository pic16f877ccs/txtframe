#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Display text in a frame.
//! Displays text within predefined or custom frames.
//! You can set the frame color, change the text alignment.
//!
//! # Examples
//!
//! ```
//! # use txtframe::*;
//! # #[cfg(feature = "color")]
//! let mut text_frame = TextFrame::new()
//!     .frame_var(&FrameVar::Space)
//!     .algn(Algn::Centr)
//!     .color_fra(Color::Red)
//!     .color_txt(Color::Cyan)
//!     .color_fill(Color::Magenta)
//!     .expand(0)
//!     .width(100)
//!     .expand_width(0)
//!     .expand_height(0)
//!     .left_top('✤')
//!     .right_top('✤')
//!     .left_btm('✤')
//!     .right_btm('✤')
//!     .top_line('―')
//!     .vert_left('│')
//!     .vert_right('│')
//!     .btm_line('―')
//!     .fill('░');
//!
//! # #[cfg(feature = "color")]
//! let text_frame_iter = text_frame.frame_iter("Text frame");
//! # #[cfg(feature = "color")]
//! println!("{}", text_frame_iter.collect::<String>());
//! ```

mod algn;
#[cfg(feature = "color")]
mod color;
mod frame_var;
mod txt_frame;

pub use crate::algn::Algn;
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use crate::color::Color;
pub use crate::frame_var::FrameVar;
pub use crate::txt_frame::TextFrame;

