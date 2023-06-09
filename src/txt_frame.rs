use crate::Algn;
#[cfg(feature = "color")]
use crate::Color;
use crate::FrameVar;
use core::iter;
use smallstr::SmallString;
#[cfg(feature = "esc")]
use strip_ansi_escapes::strip;

/// The abstract representation of a TextFrame.
///
/// # Examples
///
/// ```rust
/// # use txtframe::*;
/// # #[cfg(feature = "color")]
/// let mut text_frame = TextFrame::new()
///     .frame_var(&FrameVar::Space)
///     .algn(Algn::Centr)
///     .color_fra(Color::Red)
///     .color_txt(Color::Cyan)
///     .color_fill(Color::Magenta)
///     .expand(2)
///     .width(10)
///     .expand_width(2)
///     .expand_height(2)
///     .left_top('┏')
///     .top_line('╍')
///     .right_top('┓')
///     .vert_left('┋')
///     .vert_right('┋')
///     .left_btm('┗')
///     .btm_line('╍')
///     .right_btm('┛')
///     .fill('░');
///
/// # #[cfg(feature = "color")]
/// # #[cfg(not(feature = "newlin"))]
/// println!("{}", text_frame.frame_iter("Text frame").collect::<String>());
///
/// # #[cfg(feature = "color")]
/// # #[cfg(feature = "newlin")]
/// let chain_iters = txt_frame.frame_iterln("First frame with text")
/// .chain(txt_frame.frame_iterln("Second frame with text"));
///
/// # #[cfg(feature = "color")]
/// # #[cfg(feature = "newlin")]
/// println!("{}", chain_iters.collect::<String>());
/// ```
///
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextFrame {
    left_top_cnr: SmallString<[u8; 4]>,
    hor_top_line: SmallString<[u8; 4]>,
    right_top_cnr: SmallString<[u8; 4]>,
    left_btm_cnr: SmallString<[u8; 4]>,
    vert_left_line: SmallString<[u8; 4]>,
    vert_right_line: SmallString<[u8; 4]>,
    hor_btm_line: SmallString<[u8; 4]>,
    right_btm_cnr: SmallString<[u8; 4]>,
    fill: SmallString<[u8; 4]>,
    width: usize,
    height: usize,
    expand: usize,
    expand_width: usize,
    expand_height: usize,
    algn: Algn,
    #[cfg(feature = "color")]
    color_fra: Color,
    #[cfg(feature = "color")]
    color_txt: Color,
    #[cfg(feature = "color")]
    color_fill: Color,
}

impl TextFrame {
    /// Construct an empty frame.
    pub fn new() -> Self {
        Self {
            left_top_cnr: '┌'.into(),
            hor_top_line: '─'.into(),
            right_top_cnr: '┐'.into(),
            vert_left_line: '│'.into(),
            vert_right_line: '│'.into(),
            left_btm_cnr: '└'.into(),
            hor_btm_line: '─'.into(),
            right_btm_cnr: '┘'.into(),
            width: 0,
            height: 0,
            expand: 0,
            expand_width: 0,
            expand_height: 0,
            fill: ' '.into(),
            algn: Algn::Left,
            #[cfg(feature = "color")]
            color_fra: Color::Default,
            #[cfg(feature = "color")]
            color_txt: Color::Default,
            #[cfg(feature = "color")]
            color_fill: Color::Default,
        }
    }

    /// Selecting preset frames.
    pub fn frame_var(self, vars: &FrameVar) -> Self {
        match vars {
            FrameVar::Double => Self {
                left_top_cnr: '╔'.into(),
                hor_top_line: '═'.into(),
                right_top_cnr: '╗'.into(),
                vert_left_line: '║'.into(),
                vert_right_line: '║'.into(),
                left_btm_cnr: '╚'.into(),
                hor_btm_line: '═'.into(),
                right_btm_cnr: '╝'.into(),
                ..self
            },

            FrameVar::Space => Self {
                left_top_cnr: ' '.into(),
                hor_top_line: ' '.into(),
                right_top_cnr: ' '.into(),
                vert_left_line: ' '.into(),
                vert_right_line: ' '.into(),
                left_btm_cnr: ' '.into(),
                hor_btm_line: ' '.into(),
                right_btm_cnr: ' '.into(),
                ..self
            },

            FrameVar::HorDouble => Self {
                left_top_cnr: '╒'.into(),
                hor_top_line: '═'.into(),
                right_top_cnr: '╕'.into(),
                vert_left_line: '│'.into(),
                vert_right_line: '│'.into(),
                left_btm_cnr: '╘'.into(),
                hor_btm_line: '═'.into(),
                right_btm_cnr: '╛'.into(),
                ..self
            },

            FrameVar::VertDouble => Self {
                left_top_cnr: '╓'.into(),
                hor_top_line: '─'.into(),
                right_top_cnr: '╖'.into(),
                vert_left_line: '║'.into(),
                vert_right_line: '║'.into(),
                left_btm_cnr: '╙'.into(),
                hor_btm_line: '─'.into(),
                right_btm_cnr: '╜'.into(),
                ..self
            },

            FrameVar::Heavy => Self {
                left_top_cnr: '┏'.into(),
                hor_top_line: '━'.into(),
                right_top_cnr: '┓'.into(),
                vert_left_line: '┃'.into(),
                vert_right_line: '┃'.into(),
                left_btm_cnr: '┗'.into(),
                hor_btm_line: '━'.into(),
                right_btm_cnr: '┛'.into(),
                ..self
            },

            FrameVar::Light => Self {
                left_top_cnr: '┌'.into(),
                hor_top_line: '─'.into(),
                right_top_cnr: '┐'.into(),
                vert_left_line: '│'.into(),
                vert_right_line: '│'.into(),
                left_btm_cnr: '└'.into(),
                hor_btm_line: '─'.into(),
                right_btm_cnr: '┘'.into(),
                ..self
            },

            FrameVar::VertHeavy => Self {
                left_top_cnr: '┎'.into(),
                hor_top_line: '─'.into(),
                right_top_cnr: '┒'.into(),
                vert_left_line: '┃'.into(),
                vert_right_line: '┃'.into(),
                left_btm_cnr: '┖'.into(),
                hor_btm_line: '─'.into(),
                right_btm_cnr: '┚'.into(),
                ..self
            },

            FrameVar::HorHeavy => Self {
                left_top_cnr: '┍'.into(),
                hor_top_line: '━'.into(),
                right_top_cnr: '┑'.into(),
                vert_left_line: '│'.into(),
                vert_right_line: '│'.into(),
                left_btm_cnr: '┕'.into(),
                hor_btm_line: '━'.into(),
                right_btm_cnr: '┙'.into(),
                ..self
            },

            FrameVar::Round => Self {
                left_top_cnr: '╭'.into(),
                hor_top_line: '─'.into(),
                right_top_cnr: '╮'.into(),
                vert_left_line: '│'.into(),
                vert_right_line: '│'.into(),
                left_btm_cnr: '╰'.into(),
                hor_btm_line: '─'.into(),
                right_btm_cnr: '╯'.into(),
                ..self
            },
        }
    }

    /// Create an iterator frame with a newline.
    #[cfg(feature = "color")]
    #[cfg(feature = "newline")]
    #[cfg_attr(docsrs, doc(cfg(feature = "newline")))]
    pub fn frame_iterln<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.frame_iter(&text).chain(iter::once("\n"))
    }

    /// Create an iterator frame.
    #[cfg(feature = "color")]
    pub fn frame_iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> {
        let (lines, max_line_len) = max_line_len(text);

        let sum_exp_width = self.expand_width + self.expand;
        let max_line_len = if ((max_line_len + 2) + sum_exp_width * 2) < self.width {
            (self.width - (max_line_len + 2 + sum_exp_width * 2)) + max_line_len
        } else {
            max_line_len
        } + sum_exp_width * 2;

        let sum_exp_height = self.expand + self.expand_height;
        let sum_lines = sum_exp_height * 2 + 2 + lines;
        let take_enlarge_top = (max_line_len + 7) * sum_exp_height;

        let take_enlarge_btm = if sum_lines < self.height {
            take_enlarge_top + (max_line_len + 7) * (self.height - sum_lines)
        } else {
            take_enlarge_top
        };

        let enlarge_line_iter = iter::once(self.color_fra.into_fg_str())
            .chain(iter::once(self.vert_left_line.as_str()))
            .chain(iter::once(self.color_fill.into_fg_str()))
            .chain(iter::repeat(self.fill.as_str()).take(max_line_len))
            .chain(iter::once(self.color_fra.into_fg_str()))
            .chain(iter::once(self.vert_right_line.as_str()))
            .chain(iter::once(Color::default().into_fg_str()))
            .chain(iter::once("\n"))
            .cycle();

        let top_half_frame_iter = iter::once(self.color_fra.into_fg_str())
            .chain(iter::once(self.left_top_cnr.as_str()))
            .chain(iter::repeat(self.hor_top_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_top_cnr.as_str()))
            .chain(iter::once(Color::default().into_fg_str()))
            .chain(iter::once("\n"))
            .chain(enlarge_line_iter.clone().take(take_enlarge_top));

        let bottom_half_frame_iter = enlarge_line_iter
            .take(take_enlarge_btm)
            .chain(iter::once(self.color_fra.into_fg_str()))
            .chain(iter::once(self.left_btm_cnr.as_str()))
            .chain(iter::repeat(self.hor_btm_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_btm_cnr.as_str()))
            .chain(iter::once(Color::default().into_fg_str()));

        let lines_buffer_iter = text.lines().flat_map(move |line| {
            let curr_line_len = line.chars().count();
            let max_line_diff = max_line_len - curr_line_len;

            let alignment = match self.algn {
                Algn::Left => (sum_exp_width, max_line_diff - sum_exp_width),
                Algn::Centr => (max_line_diff / 2, max_line_diff - max_line_diff / 2),
                Algn::Right => (max_line_diff - sum_exp_width, sum_exp_width),
            };

            let iter_top = iter::once(self.color_fra.into_fg_str())
                .chain(iter::once(self.vert_left_line.as_str()))
                .chain(iter::once(self.color_fill.into_fg_str()))
                .chain(iter::repeat(self.fill.as_str()).take(alignment.0));

            let iter_line = iter::once(self.color_txt.into_fg_str()).chain(iter::once(line));

            let iter_bottom = iter::once(self.color_fill.into_fg_str())
                .chain(iter::repeat(self.fill.as_str()))
                .take(alignment.1 + 1)
                .chain(iter::once(self.color_fra.into_fg_str()))
                .chain(iter::once(self.vert_right_line.as_str()))
                .chain(iter::once(Color::default().into_fg_str()))
                .chain(iter::once("\n"));

            iter_top.chain(iter_line).chain(iter_bottom)
        });

        top_half_frame_iter
            .chain(lines_buffer_iter)
            .chain(bottom_half_frame_iter)
    }

    /// Create an iterator frame with a newline.
    #[cfg(feature = "esc")]
    #[cfg(feature = "color")]
    #[cfg(feature = "newline")]
    #[cfg_attr(docsrs, doc(cfg(feature = "newline")))]
    pub fn frame_iterln_esc<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.frame_iter(&text).chain(iter::once("\n"))
    }

    /// Create an iterator frame.
    #[cfg(feature = "esc")]
    #[cfg(feature = "color")]
    pub fn frame_iter_esc<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> + 'a {
        let (lines, max_line_len) = max_line_len_no_esc(text);

        let sum_exp_width = self.expand_width + self.expand;
        let max_line_len = if ((max_line_len + 2) + sum_exp_width * 2) < self.width {
            (self.width - (max_line_len + 2 + sum_exp_width * 2)) + max_line_len
        } else {
            max_line_len
        } + sum_exp_width * 2;

        let sum_exp_height = self.expand + self.expand_height;
        let sum_lines = sum_exp_height * 2 + 2 + lines;
        let take_enlarge_top = (max_line_len + 7) * sum_exp_height;

        let take_enlarge_btm = if sum_lines < self.height {
            take_enlarge_top + (max_line_len + 7) * (self.height - sum_lines)
        } else {
            take_enlarge_top
        };

        let enlarge_line_iter = iter::once(self.color_fra.into_fg_str())
            .chain(iter::once(self.vert_left_line.as_str()))
            .chain(iter::once(self.color_fill.into_fg_str()))
            .chain(iter::repeat(self.fill.as_str()).take(max_line_len))
            .chain(iter::once(self.color_fra.into_fg_str()))
            .chain(iter::once(self.vert_right_line.as_str()))
            .chain(iter::once(Color::default().into_fg_str()))
            .chain(iter::once("\n"))
            .cycle();

        let top_half_frame_iter = iter::once(self.color_fra.into_fg_str())
            .chain(iter::once(self.left_top_cnr.as_str()))
            .chain(iter::repeat(self.hor_top_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_top_cnr.as_str()))
            .chain(iter::once(Color::default().into_fg_str()))
            .chain(iter::once("\n"))
            .chain(enlarge_line_iter.clone().take(take_enlarge_top));

        let bottom_half_frame_iter = enlarge_line_iter
            .take(take_enlarge_btm)
            .chain(iter::once(self.color_fra.into_fg_str()))
            .chain(iter::once(self.left_btm_cnr.as_str()))
            .chain(iter::repeat(self.hor_btm_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_btm_cnr.as_str()))
            .chain(iter::once(Color::default().into_fg_str()));

        let lines_buffer_iter = text.lines().flat_map(move |line| {
            let curr_line_len = std::str::from_utf8(&strip(line).unwrap())
                .unwrap()
                .chars()
                .count();
            let max_line_diff = max_line_len - curr_line_len;

            let alignment = match self.algn {
                Algn::Left => (sum_exp_width, max_line_diff - sum_exp_width),
                Algn::Centr => (max_line_diff / 2, max_line_diff - max_line_diff / 2),
                Algn::Right => (max_line_diff - sum_exp_width, sum_exp_width),
            };

            let iter_top = iter::once(self.color_fra.into_fg_str())
                .chain(iter::once(self.vert_left_line.as_str()))
                .chain(iter::once(self.color_fill.into_fg_str()))
                .chain(iter::repeat(self.fill.as_str()).take(alignment.0));

            let iter_line = iter::once(self.color_txt.into_fg_str()).chain(iter::once(line));

            let iter_bottom = iter::once(self.color_fill.into_fg_str())
                .chain(iter::repeat(self.fill.as_str()))
                .take(alignment.1 + 1)
                .chain(iter::once(self.color_fra.into_fg_str()))
                .chain(iter::once(self.vert_right_line.as_str()))
                .chain(iter::once(Color::default().into_fg_str()))
                .chain(iter::once("\n"));

            iter_top.chain(iter_line).chain(iter_bottom)
        });

        top_half_frame_iter
            .chain(lines_buffer_iter)
            .chain(bottom_half_frame_iter)
    }

    /// Create an iterator frame.
    #[cfg(not(feature = "color"))]
    #[cfg(feature = "newline")]
    #[cfg_attr(docsrs, doc(cfg(feature = "newline")))]
    pub fn frame_iterln<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.frame_iter(&text).chain(iter::once("\n"))
    }

    /// Create an iterator frame.
    #[cfg(not(feature = "color"))]
    pub fn frame_iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> + 'a {
        let sum_exp_width = self.expand_width + self.expand;
        let (lines, max_line_len) = max_line_len(text);
        let sum_exp_height = self.expand + self.expand_height;
        let sum_lines = sum_exp_height * 2 + 2 + lines;

        let max_line_len = if ((max_line_len + 2) + sum_exp_width * 2) < self.width {
            (self.width - (max_line_len + 2 + sum_exp_width * 2)) + max_line_len
        } else {
            max_line_len
        } + sum_exp_width * 2;

        let take_enlarge_top = (max_line_len + 3) * sum_exp_height;
        let take_enlarge_btm = if sum_lines < self.height {
            (max_line_len + 3) * (self.height - sum_lines)
        } else {
            take_enlarge_top
        };

        let enlarge_line_iter = iter::once(self.vert_left_line.as_str())
            .chain(iter::repeat(self.fill.as_str()).take(max_line_len))
            .chain(iter::once(self.vert_right_line.as_str()))
            .chain(iter::once("\n"))
            .cycle();

        let top_half_frame_iter = iter::once(self.left_top_cnr.as_str())
            .chain(iter::repeat(self.hor_top_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_top_cnr.as_str()))
            .chain(iter::once("\n"))
            .chain(enlarge_line_iter.clone().take(take_enlarge_top));

        let bottom_half_frame_iter = enlarge_line_iter
            .take(take_enlarge_btm)
            .chain(iter::once(self.left_btm_cnr.as_str()))
            .chain(iter::repeat(self.hor_btm_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_btm_cnr.as_str()));

        let lines_buffer_iter = text.lines().flat_map(move |line| {
            let curr_line_len = line.chars().count();
            let max_line_diff = max_line_len - curr_line_len;

            let alignment = match self.algn {
                Algn::Left => (sum_exp_width, max_line_diff - sum_exp_width),
                Algn::Centr => (max_line_diff / 2, max_line_diff - max_line_diff / 2),
                Algn::Right => (max_line_diff - sum_exp_width, sum_exp_width),
            };

            let iter_top = iter::once(self.vert_left_line.as_str())
                .chain(iter::repeat(self.fill.as_str()).take(alignment.0));

            let iter_line = iter::once(line);

            let iter_bottom = iter::repeat(self.fill.as_str())
                .take(alignment.1)
                .chain(iter::once(self.vert_right_line.as_str()))
                .chain(iter::once("\n"));

            iter_top.chain(iter_line).chain(iter_bottom)
        });

        top_half_frame_iter
            .chain(lines_buffer_iter)
            .chain(bottom_half_frame_iter)
    }

    /// Specifies the top left corner.
    pub fn left_top(mut self, ch: char) -> Self {
        self.left_top_cnr = ch.into();
        self
    }

    /// Change top left corner.
    #[inline]
    pub fn set_left_top(&mut self, ch: char) -> &mut Self {
        self.left_top_cnr = ch.into();
        self
    }

    /// Specifies the top line.
    pub fn top_line(mut self, ch: char) -> Self {
        self.hor_top_line = ch.into();
        self
    }

    /// Change top line.
    #[inline]
    pub fn set_top_line(&mut self, ch: char) -> &mut Self {
        self.hor_top_line = ch.into();
        self
    }

    /// Specifies the top right corner.
    pub fn right_top(mut self, ch: char) -> Self {
        self.right_top_cnr = ch.into();
        self
    }

    /// Change top right corner.
    #[inline]
    pub fn set_right_top(&mut self, ch: char) -> &mut Self {
        self.right_top_cnr = ch.into();
        self
    }

    /// Specifies the left vertical line.
    pub fn vert_left(mut self, ch: char) -> Self {
        self.vert_left_line = ch.into();
        self
    }

    /// Change left vertical line.
    #[inline]
    pub fn set_vert_left(&mut self, ch: char) -> &mut Self {
        self.vert_left_line = ch.into();
        self
    }

    /// Specifies the right vertical line.
    pub fn vert_right(mut self, ch: char) -> Self {
        self.vert_right_line = ch.into();
        self
    }

    /// Change right vertical line.
    #[inline]
    pub fn set_vert_right(&mut self, ch: char) -> &mut Self {
        self.vert_right_line = ch.into();
        self
    }

    /// Specifies the bottom left corner.
    pub fn left_btm(mut self, ch: char) -> Self {
        self.left_btm_cnr = ch.into();
        self
    }

    /// Change bottom left corner.
    #[inline]
    pub fn set_left_btm(&mut self, ch: char) -> &mut Self {
        self.left_btm_cnr = ch.into();
        self
    }

    /// Specifies the bottom line.
    pub fn btm_line(mut self, ch: char) -> Self {
        self.hor_btm_line = ch.into();
        self
    }

    /// Change bottom line.
    #[inline]
    pub fn set_btm_line(&mut self, ch: char) -> &mut Self {
        self.hor_btm_line = ch.into();
        self
    }

    /// Specifies the bottom right corner.
    pub fn right_btm(mut self, ch: char) -> Self {
        self.right_btm_cnr = ch.into();
        self
    }

    /// Change bottom right corner
    #[inline]
    pub fn set_right_btm(&mut self, ch: char) -> &mut Self {
        self.right_btm_cnr = ch.into();
        self
    }

    /// Value for frame width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Change frame width.
    #[inline]
    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    /// Value for frame height.
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Change frame height.
    #[inline]
    pub fn set_height(&mut self, height: usize) -> &mut Self {
        self.height = height;
        self
    }

    /// Value for the width of the frame extension.
    pub fn expand_width(mut self, width: usize) -> Self {
        self.expand_width = width;
        self
    }

    /// Change the width of the frame extension.
    #[inline]
    pub fn set_expand_width(&mut self, width: usize) -> &mut Self {
        self.expand_width = width;
        self
    }

    /// Value for the height of the frame extension.
    pub fn expand_height(mut self, height: usize) -> Self {
        self.expand_height = height;
        self
    }

    /// Change the height of the frame extension.
    #[inline]
    pub fn set_expand_height(&mut self, height: usize) -> &mut Self {
        self.expand_height = height;
        self
    }

    /// Value for frame expand.
    pub fn expand(mut self, expand: usize) -> Self {
        self.expand = expand;
        self
    }

    /// Change frame expand.
    #[inline]
    pub fn set_expand(&mut self, expand: usize) -> &mut Self {
        self.expand = expand;
        self
    }

    /// Specifies the text alignment.
    pub fn algn(mut self, algn: Algn) -> Self {
        self.algn = algn;
        self
    }

    /// Change text alignment.
    #[inline]
    pub fn set_algn(&mut self, algn: Algn) -> &mut Self {
        self.algn = algn;
        self
    }

    /// Specifies the fill character.
    pub fn fill(mut self, fill: char) -> Self {
        self.fill = fill.into();
        self
    }

    /// Change fill character.
    #[inline]
    pub fn set_fill(&mut self, fill: char) -> &mut Self {
        self.fill = fill.into();
        self
    }

    /// Specifies the frame color.
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn color_fra(mut self, color: Color) -> Self {
        self.color_fra = color;
        self
    }

    /// Change frame color.
    #[inline]
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn set_color_fra(&mut self, color: Color) -> &mut Self {
        self.color_fra = color;
        self
    }

    /// Specifies the text color.
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn color_txt(mut self, color: Color) -> Self {
        self.color_txt = color;
        self
    }

    /// Change text color.
    #[inline]
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn set_color_txt(&mut self, color: Color) -> &mut Self {
        self.color_txt = color;
        self
    }

    /// Specifies the fill color.
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn color_fill(mut self, color: Color) -> Self {
        self.color_fill = color;
        self
    }

    /// Change fill color.
    #[inline]
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn set_color_fill(&mut self, color: Color) -> &mut Self {
        self.color_fill = color;
        self
    }
}

#[inline]
#[cfg(feature = "esc")]
fn max_line_len_no_esc(text: &str) -> (usize, usize) {
    max_line_len(std::str::from_utf8(&strip(text).unwrap()).unwrap())
}

#[inline]
fn max_line_len(text: &str) -> (usize, usize) {
    let mut line_cout = 0;
    let max_len = text
        .lines()
        .map(|line| {
            line_cout += 1;
            line.chars().count()
        })
        .max()
        .unwrap_or(0);

    (line_cout, max_len)
}
