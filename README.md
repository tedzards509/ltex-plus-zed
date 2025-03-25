# LTeX+ Zed Extension

Zed Extension to integrate the LTeX+ [language server](https://github.com/ltex-plus/ltex-ls-plus/).
LTeX+ is based on [LanguageTool](https://languagetool.org/).
Therefore, it has many features and rules, however it is slower than most other grammar check Zed extensions.

## Features

- Grammar and spell checking in the [Supported Languages](https://ltex-plus.github.io/ltex-plus/supported-languages.html) (markup, programming, human)
- Suggestions and code actions to fix errors
- A wide range of settings

## Limitations

Limitations in how the Zed extension API works:
- Can't add words to dictionaries via code actions
- Can't ignore rules via code actions
Limitations of LTeX+:
- Slow and memory intensive
- Not built to switch (human) languages on the fly
- Noticeably built for LaTeX (See table on [supported languages](https://ltex-plus.github.io/ltex-plus/supported-languages.html))

## Setup
To use this extension, download it in Zed's extension manager.
You should then be able to use it in any supported language with default settings.

To configure settings (Mainly language), you can pass settings to the LSP in either your global or your project config:
```json
{
  "lsp": {
    "ltex": {
      "settings": {
        "ltex": {
          "enabled": ["latex", "typst", "bibtex"], // May also be true to enable for all languages
          "language": "de-DE"
        }
      }
    }
  }
}
```

For all possible settings, refer to [this documentation](https://ltex-plus.github.io/ltex-plus/settings.html).
Note however that not all of these are applicable to this extension.

With these settings you can add a custom dictionary manually, and a list of rules to ignore, working around not being able to modify those via code actions:

```json
"lsp": {
  "ltex": {
    "settings": {
      "ltex": {
        "enabled": true,
        "language": "en-NZ",
        "dictionary": {
          "en-NZ": ["LTeX", "tedzards" /*, ...*/],
          "de-DE": ["Wordle", "Megohm"]
        }
      }
    }
  }
}
```


## If you've read until here
I have not found a way to get dictionary-/rules-ignored-files working, due to the containerization of LSPs, but perhaps it is possible to add and reference a file in the working directory of the extension.
Further research is welcome.
