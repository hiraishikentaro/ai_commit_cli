use anyhow::Result;
use std::io::{Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct Editor {
    content: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    message: String,
}

impl Editor {
    pub fn _new() -> Self {
        Self {
            content: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            message: String::from("-- INSERT MODE -- (Press Esc to exit, Ctrl+S to save)"),
        }
    }

    // テスト用ゲッターメソッド
    #[cfg(test)]
    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    #[cfg(test)]
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    #[cfg(test)]
    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn with_content(text: &str) -> Self {
        let content = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        };

        Self {
            content,
            cursor_x: 0,
            cursor_y: 0,
            message: String::from("-- INSERT MODE -- (Press Esc to exit, Ctrl+S to save)"),
        }
    }

    pub fn run(&mut self) -> Result<String> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

        // 画面をクリアして初期表示
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
        self.draw(&mut stdout)?;
        stdout.flush()?;

        // キー入力処理
        for c in stdin.keys() {
            match c? {
                // 終了
                Key::Esc => {
                    // 画面をクリアして終了メッセージを表示
                    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
                    write!(stdout, "Editor closed.\n\n")?;
                    stdout.flush()?;
                    break;
                }
                // 保存して終了
                Key::Ctrl('s') => {
                    self.message = String::from("Changes saved. Press Esc to exit.");
                    self.draw(&mut stdout)?;
                    stdout.flush()?;

                    // 保存内容を返す準備
                    let mut content = String::new();
                    for line in &self.content {
                        content.push_str(line);
                        content.push('\n');
                    }

                    // 画面をクリアして保存メッセージを表示
                    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
                    write!(stdout, "Changes saved. Editor closed.\n\n")?;
                    stdout.flush()?;

                    return Ok(content);
                }
                // カーソル移動
                Key::Left => {
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                    }
                }
                Key::Right => {
                    if self.cursor_x < self.content[self.cursor_y].len() {
                        self.cursor_x += 1;
                    }
                }
                Key::Up => {
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                        self.cursor_x =
                            std::cmp::min(self.cursor_x, self.content[self.cursor_y].len());
                    }
                }
                Key::Down => {
                    if self.cursor_y < self.content.len() - 1 {
                        self.cursor_y += 1;
                        self.cursor_x =
                            std::cmp::min(self.cursor_x, self.content[self.cursor_y].len());
                    }
                }
                // 改行
                Key::Char('\n') => {
                    let current_line = &self.content[self.cursor_y];
                    let new_line = if self.cursor_x < current_line.len() {
                        current_line[self.cursor_x..].to_string()
                    } else {
                        String::new()
                    };

                    if self.cursor_x < current_line.len() {
                        self.content[self.cursor_y] = current_line[..self.cursor_x].to_string();
                    }

                    self.content.insert(self.cursor_y + 1, new_line);
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
                // バックスペース
                Key::Backspace => {
                    if self.cursor_x > 0 {
                        let line = &mut self.content[self.cursor_y];
                        let mut chars: Vec<char> = line.chars().collect();
                        chars.remove(self.cursor_x - 1);
                        *line = chars.into_iter().collect();
                        self.cursor_x -= 1;
                    } else if self.cursor_y > 0 {
                        // 行の先頭で行を削除
                        let line = self.content.remove(self.cursor_y);
                        self.cursor_y -= 1;
                        self.cursor_x = self.content[self.cursor_y].len();
                        self.content[self.cursor_y].push_str(&line);
                    }
                }
                // 通常文字入力
                Key::Char(c) => {
                    if self.cursor_y >= self.content.len() {
                        self.content.push(String::new());
                    }

                    let line = &mut self.content[self.cursor_y];
                    let mut chars: Vec<char> = line.chars().collect();
                    chars.insert(self.cursor_x, c);
                    *line = chars.into_iter().collect();
                    self.cursor_x += 1;
                }
                _ => {}
            }

            // 画面を更新
            write!(stdout, "{}", clear::All)?;
            self.draw(&mut stdout)?;
            stdout.flush()?;
        }

        // 編集内容を文字列として返す
        let mut content = String::new();
        for line in &self.content {
            content.push_str(line);
            content.push('\n');
        }

        // 画面をクリアして終了メッセージを表示
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
        write!(stdout, "Editor closed. No changes saved.\n\n")?;
        stdout.flush()?;

        Ok(content)
    }

    fn draw<W: Write>(&self, stdout: &mut W) -> Result<()> {
        // ファイル内容を表示
        for (i, line) in self.content.iter().enumerate() {
            write!(stdout, "{}{}", cursor::Goto(1, (i + 1) as u16), line)?;
        }

        // ステータスメッセージを表示
        write!(
            stdout,
            "{}{}",
            cursor::Goto(1, (self.content.len() + 2) as u16),
            self.message
        )?;

        // カーソル位置を設定
        write!(
            stdout,
            "{}",
            cursor::Goto((self.cursor_x + 1) as u16, (self.cursor_y + 1) as u16)
        )?;

        Ok(())
    }

    // テスト用にテキスト処理関数を公開
    #[cfg(test)]
    pub fn process_text_for_test(&mut self, text: &str) -> String {
        // 改行をシンプルに処理するテスト用関数
        let mut lines = vec![];
        for line in text.lines() {
            lines.push(line.to_string());
        }
        self.content = lines;
        
        // テキストに行を追加
        self.content.push("Test added line".to_string());
        
        // 内容を文字列として返す
        let mut result = String::new();
        for line in &self.content {
            result.push_str(line);
            result.push('\n');
        }
        
        result
    }
}
