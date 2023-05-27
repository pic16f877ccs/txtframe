use txtframe::*;

#[cfg(not(feature = "color"))]
#[test]
fn test_default_frame_expand() {
    let txtframe = TextFrame::new().expand(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(&txtframe_iter.collect::<String>(), "┌──┐\n│  │\n│  │\n└──┘");
}

#[cfg(feature = "newline")]
#[cfg(not(feature = "color"))]
#[test]
fn test_not_color_default_frame_newline() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iterln("");

    assert_eq!(&txtframe_iter.collect::<String>(), "┌┐\n└┘\n");
}

#[cfg(not(feature = "color"))]
#[test]
fn test_default_frame_expand_width() {
    let txtframe = TextFrame::new().expand_width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(&txtframe_iter.collect::<String>(), "┌──┐\n└──┘");
}

#[cfg(not(feature = "color"))]
#[test]
fn test_not_color_default_frame() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(&txtframe_iter.collect::<String>(), "┌┐\n└┘");
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand() {
    let txtframe = TextFrame::new().expand(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Double);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╔══╗\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m╚══╝\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_hor_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::HorDouble);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╒══╕\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m╘══╛\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_vert_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::VertDouble);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╓──╖\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m╙──╜\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_round_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Round);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╭──╮\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m╰──╯\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_horheavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::HorHeavy);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┍━━┑\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m┕━━┙\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_vertheavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::VertHeavy);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┎──┒\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┖──┚\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_heavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Heavy);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┏━━┓\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┗━━┛\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_var_space_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Space);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m    \u{1b}[0m\n\u{1b}[0m \u{1b}[0m  \u{1b}[0m \u{1b}[0m\n\u{1b}[0m \u{1b}[0m  \u{1b}[0m \u{1b}[0m\n\u{1b}[0m    \u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_width() {
    let txtframe = TextFrame::new().expand_width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_red() {
    let txtframe = TextFrame::new().color_fra(Color::Red);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[31m┌┐\u{1b}[0m\n\u{1b}[31m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_blue() {
    let txtframe = TextFrame::new().expand(1).color_fra(Color::Blue);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[34m┌──┐\u{1b}[0m\n\u{1b}[34m│\u{1b}[0m  \u{1b}[34m│\u{1b}[0m\n\u{1b}[34m│\u{1b}[0m  \u{1b}[34m│\u{1b}[0m\n\u{1b}[34m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_fill_green() {
    let txtframe = TextFrame::new().expand(1).fill('░').color_fra(Color::Green);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[32m┌──┐\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_cyan_expand_fill_magenta() {
    let txtframe = TextFrame::new()
        .expand(1)
        .fill('░')
        .color_fra(Color::Cyan)
        .color_fill(Color::Magenta);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[36m┌──┐\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_text() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "newline")]
#[cfg(feature = "color")]
#[test]
fn test_default_frame_text_newline() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iterln("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m\n"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_height() {
    let txtframe = TextFrame::new().height(3);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_text_height() {
    let txtframe = TextFrame::new().height(5);
    let txtframe_iter = txtframe.frame_iter("Text Frame");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
"\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m          \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m          \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m");
}

#[test]
fn test_default_frame_text_line_count_four() {
    let txtframe = TextFrame::new().height(4);
    let txtframe_iter = txtframe.frame_iter("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 4);
}

#[test]
fn test_default_frame_text_line_count_height_five() {
    let txtframe = TextFrame::new().height(5);
    let txtframe_iter = txtframe.frame_iter("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 5);
}

#[test]
fn test_default_frame_text_line_count_height_ten() {
    let txtframe = TextFrame::new().height(10);
    let txtframe_iter = txtframe.frame_iter("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 10);
}

#[test]
fn test_default_frame_text_line_count_height_zero() {
    let txtframe = TextFrame::new().height(0);
    let txtframe_iter = txtframe.frame_iter("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 4);
}

#[test]
fn test_default_frame_text_line_count_height_zero_empy() {
    let txtframe = TextFrame::new().height(0);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 2);
}

#[test]
fn test_default_frame_text_line_count_newline() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter("\n");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 3);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_height_lt() {
    let txtframe = TextFrame::new().height(2);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_width() {
    let txtframe = TextFrame::new().width(3);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌─┐\u{1b}[0m\n\u{1b}[0m└─┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_width_lt() {
    let txtframe = TextFrame::new().width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_text_width() {
    let txtframe = TextFrame::new().width(13);
    let txtframe_iter = txtframe.frame_iter("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌───────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└───────────┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_text_color() {
    let txtframe = TextFrame::new().color_txt(Color::Cyan);
    let txtframe_iter = txtframe.frame_iter("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[36mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_set_height() {
    let mut txtframe = TextFrame::new();
    txtframe.set_height(3);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_set_expand_width() {
    let mut txtframe = TextFrame::new();
    txtframe.set_expand_width(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_set_expand() {
    let mut txtframe = TextFrame::new();
    txtframe.set_expand(1);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_set_red() {
    let mut txtframe = TextFrame::new();
    txtframe.set_color_fra(Color::Red);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[31m┌┐\u{1b}[0m\n\u{1b}[31m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_set_text_color() {
    let mut txtframe = TextFrame::new();
    txtframe.set_color_txt(Color::Cyan);
    let txtframe_iter = txtframe.frame_iter("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[36mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_cyan_expand_fill_set_magenta() {
    let mut txtframe = TextFrame::new().expand(1).color_fra(Color::Cyan).fill('░');
    txtframe.set_color_fill(Color::Magenta);
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[36m┌──┐\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "color")]
#[test]
fn test_default_frame_expand_set_fill_green() {
    let mut txtframe = TextFrame::new().expand(1).color_fra(Color::Green);
    txtframe.set_fill('░');
    let txtframe_iter = txtframe.frame_iter("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[32m┌──┐\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_expand() {
    let txtframe = TextFrame::new().expand(1);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Double);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╔══╗\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m╚══╝\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_hor_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::HorDouble);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╒══╕\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m╘══╛\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_vert_double_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::VertDouble);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╓──╖\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m║\u{1b}[0m  \u{1b}[0m║\u{1b}[0m\n\u{1b}[0m╙──╜\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_round_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Round);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m╭──╮\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m╰──╯\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_horheavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::HorHeavy);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┍━━┑\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m┕━━┙\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_vertheavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::VertHeavy);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┎──┒\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┖──┚\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_heavy_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Heavy);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┏━━┓\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┃\u{1b}[0m  \u{1b}[0m┃\u{1b}[0m\n\u{1b}[0m┗━━┛\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_var_space_expand() {
    let txtframe = TextFrame::new().expand(1).frame_var(&FrameVar::Space);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m    \u{1b}[0m\n\u{1b}[0m \u{1b}[0m  \u{1b}[0m \u{1b}[0m\n\u{1b}[0m \u{1b}[0m  \u{1b}[0m \u{1b}[0m\n\u{1b}[0m    \u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_expand_width() {
    let txtframe = TextFrame::new().expand_width(1);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_red() {
    let txtframe = TextFrame::new().color_fra(Color::Red);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[31m┌┐\u{1b}[0m\n\u{1b}[31m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_expand_blue() {
    let txtframe = TextFrame::new().expand(1).color_fra(Color::Blue);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[34m┌──┐\u{1b}[0m\n\u{1b}[34m│\u{1b}[0m  \u{1b}[34m│\u{1b}[0m\n\u{1b}[34m│\u{1b}[0m  \u{1b}[34m│\u{1b}[0m\n\u{1b}[34m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_expand_fill_green() {
    let txtframe = TextFrame::new().expand(1).fill('░').color_fra(Color::Green);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[32m┌──┐\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_cyan_expand_fill_magenta() {
    let txtframe = TextFrame::new()
        .expand(1)
        .fill('░')
        .color_fra(Color::Cyan)
        .color_fill(Color::Magenta);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[36m┌──┐\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_text() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter_esc("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "newline")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_text_newline() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iterln("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m\n"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_height() {
    let txtframe = TextFrame::new().height(3);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_text_height() {
    let txtframe = TextFrame::new().height(5);
    let txtframe_iter = txtframe.frame_iter_esc("Text Frame");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
"\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m          \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m          \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m");
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_four() {
    let txtframe = TextFrame::new().height(4);
    let txtframe_iter = txtframe.frame_iter_esc("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 4);
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_height_five() {
    let txtframe = TextFrame::new().height(5);
    let txtframe_iter = txtframe.frame_iter_esc("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 5);
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_height_ten() {
    let txtframe = TextFrame::new().height(10);
    let txtframe_iter = txtframe.frame_iter_esc("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 10);
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_height_zero() {
    let txtframe = TextFrame::new().height(0);
    let txtframe_iter = txtframe.frame_iter_esc("First line\nsecond line.");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 4);
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_height_zero_empy() {
    let txtframe = TextFrame::new().height(0);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 2);
}

#[cfg(feature = "esc")]
#[test]
fn esc_test_default_frame_text_line_count_newline() {
    let txtframe = TextFrame::new();
    let txtframe_iter = txtframe.frame_iter_esc("\n");

    assert_eq!(txtframe_iter.collect::<String>().lines().count(), 3);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_height_lt() {
    let txtframe = TextFrame::new().height(2);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_width() {
    let txtframe = TextFrame::new().width(3);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌─┐\u{1b}[0m\n\u{1b}[0m└─┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_width_lt() {
    let txtframe = TextFrame::new().width(1);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_text_width() {
    let txtframe = TextFrame::new().width(13);
    let txtframe_iter = txtframe.frame_iter_esc("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌───────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0mText Frame\u{1b}[0m \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└───────────┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_text_color() {
    let txtframe = TextFrame::new().color_txt(Color::Cyan);
    let txtframe_iter = txtframe.frame_iter_esc("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[36mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_set_height() {
    let mut txtframe = TextFrame::new();
    txtframe.set_height(3);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_set_expand_width() {
    let mut txtframe = TextFrame::new();
    txtframe.set_expand_width(1);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_set_expand() {
    let mut txtframe = TextFrame::new();
    txtframe.set_expand(1);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m  \u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_set_red() {
    let mut txtframe = TextFrame::new();
    txtframe.set_color_fra(Color::Red);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
        &txtframe_iter.collect::<String>(),
        "\u{1b}[31m┌┐\u{1b}[0m\n\u{1b}[31m└┘\u{1b}[0m"
    );
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_set_text_color() {
    let mut txtframe = TextFrame::new();
    txtframe.set_color_txt(Color::Cyan);
    let txtframe_iter = txtframe.frame_iter_esc("Text Frame");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[0m┌──────────┐\u{1b}[0m\n\u{1b}[0m│\u{1b}[0m\u{1b}[36mText Frame\u{1b}[0m\u{1b}[0m│\u{1b}[0m\n\u{1b}[0m└──────────┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_cyan_expand_fill_set_magenta() {
    let mut txtframe = TextFrame::new().expand(1).color_fra(Color::Cyan).fill('░');
    txtframe.set_color_fill(Color::Magenta);
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[36m┌──┐\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m│\u{1b}[35m░░\u{1b}[36m│\u{1b}[0m\n\u{1b}[36m└──┘\u{1b}[0m"
);
}

#[cfg(feature = "esc")]
#[cfg(feature = "color")]
#[test]
fn esc_test_default_frame_expand_set_fill_green() {
    let mut txtframe = TextFrame::new().expand(1).color_fra(Color::Green);
    txtframe.set_fill('░');
    let txtframe_iter = txtframe.frame_iter_esc("");

    assert_eq!(
    &txtframe_iter.collect::<String>(),
    "\u{1b}[32m┌──┐\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m│\u{1b}[0m░░\u{1b}[32m│\u{1b}[0m\n\u{1b}[32m└──┘\u{1b}[0m"
);
}
