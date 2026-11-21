#![allow(dead_code)]

//! Bordered text tables that shrink to the terminal.

use super::color::Style;

const BORDER: (u8, u8, u8) = (0x60, 0x72, 0x95);
const DEFAULT_MAX: usize = 32;

/// Aligns text inside a column.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Align {
    /// Pads on the right.
    #[default]
    Left,
    /// Pads on the left.
    Right,
    /// Pads on both sides.
    Center,
}

/// Text shown in one table cell.
///
/// `&str` and `String` become an uncolored, left-aligned cell.
#[derive(Clone)]
pub struct Cell {
    text: String,
    color: Option<(u8, u8, u8)>,
    align: Align,
}

impl Cell {
    fn new(text: impl Into<String>, color: Option<(u8, u8, u8)>, align: Align) -> Self {
        Self {
            text: text.into(),
            color,
            align,
        }
    }

    /// Creates an uncolored, left-aligned cell.
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(text, None, Align::Left)
    }

    /// Creates a left-aligned cell. `color` is `(red, green, blue)`.
    pub fn colored(text: impl Into<String>, color: (u8, u8, u8)) -> Self {
        Self::new(text, Some(color), Align::Left)
    }

    /// Aligns this cell to the left.
    #[must_use]
    pub fn left(mut self) -> Self {
        self.align = Align::Left;
        self
    }

    /// Aligns this cell to the right.
    #[must_use]
    pub fn right(mut self) -> Self {
        self.align = Align::Right;
        self
    }

    /// Centers this cell in its column.
    #[must_use]
    pub fn center(mut self) -> Self {
        self.align = Align::Center;
        self
    }
}

impl From<&str> for Cell {
    fn from(text: &str) -> Self {
        Self::text(text)
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Self::text(text)
    }
}

/// A bordered grid. Columns come from the headers.
///
/// Wider cells end in `…`. Columns shrink from the right when the grid is wider than the terminal.
///
/// # Examples
///
/// ```
/// use crate::utils::{Cell, Table};
///
/// let mut table = Table::new(["ID", "NAME"]).width(0, 4, 8);
/// table.push([Cell::text("1").right(), "calx"]);
/// table.print();
/// ```
#[must_use = "call `print` or `render` to show the table"]
pub struct Table {
    headers: Vec<Cell>,
    rows: Vec<Vec<Cell>>,
    min: Vec<usize>,
    max: Vec<usize>,
    border: (u8, u8, u8),
    flow: bool,
}

impl Table {
    /// Creates a table with one column per header.
    pub fn new(headers: impl IntoIterator<Item = impl Into<Cell>>) -> Self {
        let headers: Vec<Cell> = headers.into_iter().map(Into::into).collect();
        let n = headers.len();
        Self {
            min: headers.iter().map(|h| h.text.chars().count()).collect(),
            max: vec![DEFAULT_MAX; n],
            headers,
            rows: Vec::new(),
            border: BORDER,
            flow: false,
        }
    }

    /// Sets column `col` to at least `min` and at most `max` characters.
    ///
    /// A `col` past the last header is ignored. `max` is raised to `min` when it is smaller.
    #[must_use]
    pub fn width(mut self, col: usize, min: usize, max: usize) -> Self {
        if let Some(slot) = self.min.get_mut(col) {
            *slot = min;
        }
        if let Some(slot) = self.max.get_mut(col) {
            *slot = max.max(min);
        }
        self
    }

    /// Sets the frame color. `color` is `(red, green, blue)`.
    #[must_use]
    pub fn border(mut self, color: (u8, u8, u8)) -> Self {
        self.border = color;
        self
    }

    /// Repeats the columns side by side, up to 6 panels.
    #[must_use]
    pub fn flow(mut self) -> Self {
        self.flow = true;
        self
    }

    /// Appends one row. Extra cells are dropped and missing cells are blank.
    pub fn push<I, C>(&mut self, cells: I)
    where
        I: IntoIterator<Item = C>,
        C: Into<Cell>,
    {
        let n = self.headers.len();
        let mut row: Vec<Cell> = cells.into_iter().map(Into::into).take(n).collect();
        row.resize_with(n, || Cell::text(""));
        self.rows.push(row);
    }

    /// Renders the grid. ANSI color is included when stdout is a terminal.
    pub fn render(&self) -> String {
        self.layout(terminal_width()).join("\n")
    }

    /// Prints the grid to stdout, with one blank line above it.
    pub fn print(&self) {
        let body = self.render();
        if !body.is_empty() {
            println!("\n{body}");
        }
    }

    fn layout(&self, term: usize) -> Vec<String> {
        if self.headers.is_empty() {
            return Vec::new();
        }

        let widths = self.widths(term);
        let panels = if self.flow && !self.rows.is_empty() {
            let table_w = frame_width(&widths);
            ((term + 1) / (table_w + 1)).clamp(1, self.rows.len().min(6))
        } else {
            1
        };
        let chunk = if self.rows.is_empty() {
            0
        } else {
            self.rows.len().div_ceil(panels)
        };

        let style = Style::stdout();
        let tables: Vec<_> = (0..panels)
            .map(|i| {
                let start = i * chunk;
                let end = (start + chunk).min(self.rows.len());
                draw(
                    &style,
                    self.border,
                    &self.headers,
                    &self.rows[start..end],
                    &widths,
                )
            })
            .collect();

        let blank = " ".repeat(frame_width(&widths));
        let height = tables.iter().map(Vec::len).max().unwrap_or(0);
        (0..height)
            .map(|line| {
                tables
                    .iter()
                    .map(|t| t.get(line).map(String::as_str).unwrap_or(&blank))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect()
    }

    fn widths(&self, term: usize) -> Vec<usize> {
        let mut widths: Vec<_> = (0..self.headers.len())
            .map(|i| {
                let content = self
                    .rows
                    .iter()
                    .map(|row| escape(&row[i].text).chars().count())
                    .max()
                    .unwrap_or(0);
                escape(&self.headers[i].text)
                    .chars()
                    .count()
                    .max(content)
                    .max(self.min[i])
                    .min(self.max[i].max(self.min[i]))
            })
            .collect();

        while frame_width(&widths) > term {
            let Some(i) = (0..widths.len()).rev().find(|&i| widths[i] > 1) else {
                break;
            };
            widths[i] -= 1;
        }
        widths
    }
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\r' => {}
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

fn fit(text: &str, width: usize, align: Align) -> String {
    let escaped = escape(text);
    let mut shown: String = escaped.chars().take(width).collect();
    if escaped.chars().count() > width && width > 0 {
        shown.pop();
        shown.push('…');
    }
    let pad = width.saturating_sub(shown.chars().count());
    let (left, right) = match align {
        Align::Left => (0, pad),
        Align::Right => (pad, 0),
        Align::Center => (pad / 2, pad - pad / 2),
    };
    format!("{}{shown}{}", " ".repeat(left), " ".repeat(right))
}

fn frame_width(widths: &[usize]) -> usize {
    widths.iter().sum::<usize>() + widths.len() * 3 + 1
}

fn rule(widths: &[usize], left: char, mid: char, right: char) -> String {
    let mut s = String::from(left);
    for (i, w) in widths.iter().enumerate() {
        s.extend(std::iter::repeat_n('─', w + 2));
        s.push(if i + 1 == widths.len() { right } else { mid });
    }
    s
}

fn draw(
    style: &Style,
    border: (u8, u8, u8),
    headers: &[Cell],
    rows: &[Vec<Cell>],
    widths: &[usize],
) -> Vec<String> {
    let b = |text: &str| style.rgb(border, text);
    let paint = |cells: Vec<(String, Option<(u8, u8, u8)>)>| {
        let mut line = b("│");
        for (text, color) in cells {
            line.push(' ');
            line.push_str(&match color {
                Some(color) => style.rgb(color, &text),
                None => text,
            });
            line.push(' ');
            line.push_str(&b("│"));
        }
        line
    };

    let mut lines = vec![
        b(&rule(widths, '┌', '┬', '┐')),
        paint(
            headers
                .iter()
                .zip(widths)
                .map(|(cell, w)| {
                    (
                        fit(&cell.text, *w, Align::Left),
                        Some(cell.color.unwrap_or(border)),
                    )
                })
                .collect(),
        ),
    ];
    if rows.is_empty() {
        lines.push(b(&rule(widths, '└', '┴', '┘')));
        return lines;
    }

    lines.push(b(&rule(widths, '├', '┼', '┤')));
    for (i, row) in rows.iter().enumerate() {
        lines.push(paint(
            row.iter()
                .zip(widths)
                .map(|(cell, w)| (fit(&cell.text, *w, cell.align), cell.color))
                .collect(),
        ));
        let (l, m, r) = if i + 1 == rows.len() {
            ('└', '┴', '┘')
        } else {
            ('├', '┼', '┤')
        };
        lines.push(b(&rule(widths, l, m, r)));
    }
    lines
}

fn terminal_width() -> usize {
    if let Some(n) = std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    {
        return n.max(20);
    }
    #[cfg(windows)]
    if let Some(n) = windows_width() {
        return n.max(20);
    }
    80
}

#[cfg(windows)]
fn windows_width() -> Option<usize> {
    #[repr(C)]
    struct Info {
        _pad: [i16; 4],
        _attrs: u16,
        window: [i16; 4],
        _max: [i16; 2],
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetStdHandle(n: u32) -> *mut std::ffi::c_void;
        fn GetConsoleScreenBufferInfo(h: *mut std::ffi::c_void, i: *mut Info) -> i32;
    }
    unsafe {
        let h = GetStdHandle(0xFFFF_FFF5);
        if h.is_null() || h == (-1isize as *mut _) {
            return None;
        }
        let mut info = std::mem::zeroed::<Info>();
        (GetConsoleScreenBufferInfo(h, &mut info) != 0)
            .then(|| (info.window[2] - info.window[0] + 1) as usize)
            .filter(|&w| w > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clips_and_aligns() {
        let mut table = Table::new(["ID", "NAME", "NOTE"])
            .width(0, 4, 4)
            .width(1, 6, 6)
            .width(2, 4, 8);
        table.push([
            Cell::text("42").right(),
            Cell::text("abcdefghij"),
            Cell::text("a\tb"),
        ]);
        table.push(["x"]);

        let lines = table.layout(200);
        assert_eq!(lines[0], "┌──────┬────────┬──────┐");
        assert!(lines[3].contains("  42"));
        assert!(lines[3].contains("abcde…"));
        assert!(lines[3].contains("a\\tb"));
        assert!(lines.iter().any(|l| l.contains('x')));
    }
}
