use crate::Algn;
#[cfg(feature = "color")]
use crate::Color;
use crate::FrameVar;
use core::iter;
use smallstr::SmallString;

/// The abstract representation of a TextFrame.
///
/// # Examples
///
/// ```rust
/// # use txtframe::*;
///
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

    /// Select preset frames.
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

    /// Create an iterator frame.
    pub fn frame_iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &str> {
        let sum_expands = self.expand_width + self.expand;
        let mut max_line_len = max_line_len(text);
        max_line_len = if max_line_len < self.width {
            self.width
        } else {
            max_line_len
        } + sum_expands * 2;

        #[cfg(feature = "color")]
        let enlarge_line_iter = iter::once(self.vert_left_line.as_str())
            .chain(iter::once(self.color_fill.into_fg_str()))
            .chain(iter::repeat(self.fill.as_str()).take(max_line_len))
            .chain(iter::once(self.color_fra.into_fg_str()))
            .chain(iter::once(self.vert_right_line.as_str()))
            .chain(iter::once("\n"))
            .cycle()
            .take((max_line_len + 5) * (self.expand + self.expand_height));

        #[cfg(not(feature = "color"))]
        let enlarge_line_iter = iter::once(self.vert_left_line.as_str())
            .chain(iter::repeat(self.fill.as_str()).take(max_line_len))
            .chain(iter::once(self.vert_right_line.as_str()))
            .chain(iter::once("\n"))
            .cycle()
            .take((max_line_len + 3) * (self.expand + self.expand_height));

        #[cfg(feature = "color")]
        let top_half_frame_iter = iter::once(self.color_fra.into_fg_str())
            .chain(iter::once(self.left_top_cnr.as_str()))
            .chain(iter::repeat(self.hor_top_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_top_cnr.as_str()))
            .chain(iter::once("\n"))
            .chain(enlarge_line_iter.clone());

        #[cfg(not(feature = "color"))]
        let top_half_frame_iter = iter::once(self.left_top_cnr.as_str())
            .chain(iter::repeat(self.hor_top_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_top_cnr.as_str()))
            .chain(iter::once("\n"))
            .chain(enlarge_line_iter.clone());

        #[cfg(feature = "color")]
        let bottom_half_frame_iter = (enlarge_line_iter)
            .chain(iter::once(self.left_btm_cnr.as_str()))
            .chain(iter::repeat(self.hor_btm_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_btm_cnr.as_str()))
            .chain(iter::once(Color::default().into_fg_str()));

        #[cfg(not(feature = "color"))]
        let bottom_half_frame_iter = (enlarge_line_iter)
            .chain(iter::once(self.left_btm_cnr.as_str()))
            .chain(iter::repeat(self.hor_btm_line.as_str()).take(max_line_len))
            .chain(iter::once(self.right_btm_cnr.as_str()));

        #[cfg(feature = "color")]
        let lines_buffer_iter = text.lines().flat_map(move |line| {
            let curr_line_len = line.chars().count();
            let max_line_diff = max_line_len - curr_line_len;

            let alignment = match self.algn {
                Algn::Left => (sum_expands, max_line_diff - sum_expands),
                Algn::Centr => (max_line_diff / 2, max_line_diff - max_line_diff / 2),
                Algn::Right => (max_line_diff - sum_expands, sum_expands),
            };

            let iter_top = iter::once(self.vert_left_line.as_str())
                .chain(iter::once(self.color_fill.into_fg_str()))
                .chain(iter::repeat(self.fill.as_str()).take(alignment.0));

            let iter_line = iter::once(self.color_txt.into_fg_str()).chain(iter::once(line));

            let iter_bottom = iter::once(self.color_fill.into_fg_str())
                .chain(iter::repeat(self.fill.as_str()))
                .take(alignment.1 + 1)
                .chain(iter::once(self.color_fra.into_fg_str()))
                .chain(iter::once(self.vert_right_line.as_str()))
                .chain(iter::once("\n"));

            iter_top.chain(iter_line).chain(iter_bottom)
        });

        #[cfg(not(feature = "color"))]
        let lines_buffer_iter = text.lines().flat_map(move |line| {
            let curr_line_len = line.chars().count();
            let max_line_diff = max_line_len - curr_line_len;

            let alignment = match self.algn {
                Algn::Left => (sum_expands, max_line_diff - sum_expands),
                Algn::Centr => (max_line_diff / 2, max_line_diff - max_line_diff / 2),
                Algn::Right => (max_line_diff - sum_expands, sum_expands),
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

    /// Value for top left corner.
    pub fn left_top(mut self, ch: char) -> Self {
        self.left_top_cnr = ch.into();
        self
    }

    /// Value for top line.
    pub fn top_line(mut self, ch: char) -> Self {
        self.hor_top_line = ch.into();
        self
    }

    /// Value for top right corner.
    pub fn right_top(mut self, ch: char) -> Self {
        self.right_top_cnr = ch.into();
        self
    }

    /// Value for left vertical line.
    pub fn vert_left(mut self, ch: char) -> Self {
        self.vert_left_line = ch.into();
        self
    }

    /// Value for right vertical line.
    pub fn vert_right(mut self, ch: char) -> Self {
        self.vert_right_line = ch.into();
        self
    }

    /// Value for bottom left corner.
    pub fn left_btm(mut self, ch: char) -> Self {
        self.left_btm_cnr = ch.into();
        self
    }

    /// Value for bottom line.
    pub fn btm_line(mut self, ch: char) -> Self {
        self.hor_btm_line = ch.into();
        self
    }

    /// Value for bottom right corner.
    pub fn right_btm(mut self, ch: char) -> Self {
        self.right_btm_cnr = ch.into();
        self
    }

    /// Change top left corner.
    pub fn set_left_top(&mut self, ch: char) -> &mut Self {
        self.left_top_cnr = ch.into();
        self
    }

    /// Change top line.
    pub fn set_top_line(&mut self, ch: char) -> &mut Self {
        self.hor_top_line = ch.into();
        self
    }

    /// Change top right corner.
    pub fn set_right_top(&mut self, ch: char) -> &mut Self {
        self.right_top_cnr = ch.into();
        self
    }

    /// Change left vertical line.
    pub fn set_vert_left(&mut self, ch: char) -> &mut Self {
        self.vert_left_line = ch.into();
        self
    }

    /// Change right vertical line.
    pub fn set_vert_right(&mut self, ch: char) -> &mut Self {
        self.vert_right_line = ch.into();
        self
    }

    /// Change bottom left corner.
    pub fn set_left_btm(&mut self, ch: char) -> &mut Self {
        self.left_btm_cnr = ch.into();
        self
    }

    /// Change bottom line.
    pub fn set_btm_line(&mut self, ch: char) -> &mut Self {
        self.hor_btm_line = ch.into();
        self
    }

    /// Change bottom right corner.
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
    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    /// Value for frame width expand.
    pub fn expand_width(mut self, width: usize) -> Self {
        self.expand_width = width;
        self
    }

    /// Change the Expand Width value.
    pub fn set_expand_width(&mut self, width: usize) -> &mut Self {
        self.expand_width = width;
        self
    }

    /// Value for frame height expand.
    pub fn expand_height(mut self, height: usize) -> Self {
        self.expand_height = height;
        self
    }

    /// Change the Expand Height value.
    pub fn set_expand_height(&mut self, height: usize) -> &mut Self {
        self.expand_height = height;
        self
    }

    /// Value for frame expand.
    pub fn expand(mut self, expand: usize) -> Self {
        self.expand = expand;
        self
    }

    /// Change the expand value.
    pub fn set_expand(&mut self, expand: usize) -> &mut Self {
        self.expand = expand;
        self
    }

    /// Value for align expand.
    pub fn algn(mut self, algn: Algn) -> Self {
        self.algn = algn;
        self
    }

    /// Change the alignment value.
    pub fn set_algn(&mut self, algn: Algn) -> &Self {
        self.algn = algn;
        self
    }

    /// Value for fill expand.
    pub fn fill(mut self, fill: char) -> Self {
        self.fill = fill.into();
        self
    }

    /// Change the fill value.
    pub fn set_fill(&mut self, fill: char) -> &mut Self {
        self.fill = fill.into();
        self
    }

    /// Specifies the frame color.
    #[cfg(feature = "color")]
    pub fn color_fra(mut self, color: Color) -> Self {
        self.color_fra = color;
        self
    }

    /// Change frame color.
    #[cfg(feature = "color")]
    pub fn set_color_fra(&mut self, color: Color) -> &mut Self {
        self.color_fra = color;
        self
    }

    /// Specifies the text color.
    #[cfg(feature = "color")]
    pub fn color_txt(mut self, color: Color) -> Self {
        self.color_txt = color;
        self
    }

    /// Change text color.
    #[cfg(feature = "color")]
    pub fn set_color_txt(&mut self, color: Color) -> &mut Self {
        self.color_txt = color;
        self
    }

    /// Specifies the fill color.
    #[cfg(feature = "color")]
    pub fn color_fill(mut self, color: Color) -> Self {
        self.color_fill = color;
        self
    }

    /// Change fill color.
    #[cfg(feature = "color")]
    pub fn set_color_fill(&mut self, color: Color) -> &mut Self {
        self.color_fill = color;
        self
    }
}

fn max_line_len(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\n└┘\u{1b}[0m"
    );
}

#[cfg(not(feature = "color"))]
#[test]
fn test_default_frame() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "┌┐\n└┘"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand() {
    let txtframe = TextFrame::new().expand(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\n│\u{1b}[0m  \u{1b}[0m│\n│\u{1b}[0m  \u{1b}[0m│\n└──┘\u{1b}[0m"
    );
}

#[cfg(not(feature = "color"))]
#[test]
fn test_default_frame_expand() {
    let txtframe = TextFrame::new().expand(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "┌──┐\n│  │\n│  │\n└──┘"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_width() {
    let txtframe = TextFrame::new().expand_width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\n└──┘\u{1b}[0m"
    );
}

#[cfg(not(feature = "color"))]
#[test]
fn test_default_frame_expand_width() {
    let txtframe = TextFrame::new().expand_width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "┌──┐\n└──┘"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_red() {
    let txtframe = TextFrame::new().color_fra(Color::Red);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[31m┌┐\n└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_blue() {
    let txtframe = TextFrame::new().expand(1).color_fra(Color::Blue);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[34m┌──┐\n│\u{1b}[0m  \u{1b}[34m│\n│\u{1b}[0m  \u{1b}[34m│\n└──┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_fill_green() {
    let txtframe = TextFrame::new().expand(1).fill('░').color_fra(Color::Green);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[32m┌──┐\n│\u{1b}[0m░░\u{1b}[32m│\n│\u{1b}[0m░░\u{1b}[32m│\n└──┘\u{1b}[0m"
    );
}

