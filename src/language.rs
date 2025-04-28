use serde::{Deserialize, Serialize};
use promptuity::{prompts::{Select, SelectOption}, themes::FancyTheme, Promptuity, Term};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Language {
    Japanese,
    English,
    Chinese,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Japanese => "Japanese",
            Language::English => "English",
            Language::Chinese => "Chinese",
        }
    }

    pub fn system_prompt(&self) -> &'static str {
        match self {
            Language::Japanese => "あなたは優れたコミットメッセージを作成するエキスパートです。以下のGitの差分に基づいて、以下のフォーマットに従ったコンパクトで明確なコミットメッセージを生成してください。
```
フォーマット:
<変更の要約（50文字以内が望ましい）> \n\n
<必要に応じて変更の詳細な説明（各行72文字以内が望ましい>

良いコミットメッセージの特徴:
1. 簡潔で明確
2. 何が変更されたかではなく「なぜ」変更されたかを説明
3. 関連する課題やバグ修正への参照を含める
```
コミットメッセージは日本語で生成してください。",
            Language::English => "You are an expert at creating commit messages. Based on the following Git diff, generate a compact and clear commit message following the format below.
```
Format:
<Summary of the change (preferably under 50 characters)> \n\n
<Detailed explanation of the change if necessary (each line preferably under 72 characters)>

Characteristics of a good commit message:
1. Concise and clear
2. Explains 'why' the change was made, not just what was changed
3. Includes references to related issues or bug fixes
```
Please generate the commit message in English.",
            Language::Chinese => "You are an expert at creating commit messages. Based on the following Git diff, generate a compact and clear commit message following the format below.

```
Format:
<Summary of the change (preferably under 50 characters)> \n\n
<Detailed explanation of the change if necessary (each line preferably under 72 characters)>
```

Please generate the commit message in Chinese.",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Japanese
    }
}

pub fn select_language() -> anyhow::Result<Language> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let options = vec![
        ("Japanese (日本語)", Language::Japanese),
        ("English", Language::English),
        ("Chinese (中文)", Language::Chinese),
    ];

    let select_options: Vec<SelectOption<String>> = options
        .iter()
        .map(|(label, _)| SelectOption::new(label.to_string(), label.to_string()))
        .collect();

    let mut select = Select::new("Select language for commit messages", select_options);

    p.begin()?;
    let selected = p.prompt(&mut select)?;
    p.finish()?;

    // Find the matching language based on the selected label
    let selected_language = options
        .iter()
        .find(|(label, _)| label == &selected)
        .map(|(_, lang)| *lang)
        .unwrap_or(Language::default());

    Ok(selected_language)
}
