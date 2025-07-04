use libtiny_common::{MsgSource, TabStyle};
use termbox_simple::{TB_UNDERLINE, Termbox};

use unicode_width::UnicodeWidthStr;

use crate::{
    config::{Colors, Style},
    messaging::MessagingUI,
};

pub(crate) struct Tab {
    pub(crate) visible_name: String,
    pub(crate) widget: MessagingUI,
    pub(crate) src: MsgSource,
    pub(crate) style: TabStyle,
    /// Alt-character to use to switch to this tab.
    pub(crate) switch: Option<char>,
}

fn tab_style(style: TabStyle, colors: &Colors) -> Style {
    match style {
        TabStyle::Normal => colors.tab_normal,
        TabStyle::JoinOrPart => colors.tab_joinpart,
        TabStyle::NewMsg => colors.tab_new_msg,
        TabStyle::Highlight => colors.tab_highlight,
    }
}

impl Tab {
    pub(crate) fn visible_name(&self) -> &str {
        &self.visible_name
    }

    pub(crate) fn set_style(&mut self, style: TabStyle) {
        self.style = style;
    }

    pub(crate) fn update_source<F>(&mut self, f: &F)
    where
        F: Fn(&mut MsgSource),
    {
        f(&mut self.src)
    }

    pub(crate) fn width(&self) -> i32 {
        self.visible_name().width() as i32
    }

    pub(crate) fn draw(
        &self,
        tb: &mut Termbox,
        colors: &Colors,
        mut pos_x: i32,
        pos_y: i32,
        active: bool,
    ) {
        let style: Style = if active {
            colors.tab_active
        } else {
            tab_style(self.style, colors)
        };

        let mut switch_drawn = false;
        for ch in self.visible_name().chars() {
            if Some(ch) == self.switch && !switch_drawn {
                tb.change_cell(pos_x, pos_y, ch, style.fg | TB_UNDERLINE, style.bg);
                switch_drawn = true;
            } else {
                tb.change_cell(pos_x, pos_y, ch, style.fg, style.bg);
            }
            pos_x += 1;
        }
    }
}
