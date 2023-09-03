# Carryover Script for Obsidian Daily Notes To-Dos
Rust script for the [Templater](https://github.com/SilentVoid13/Templater) [Obsidian](https://obsidian.md) plugin to carryover uncompleted To-dos from
my vault's daily notes.

My Daily notes have a section looking like this when filled:
```markdown
---
filename: 2023-08-31.md
---

## 📌 Today's Goals
 
**Subsection A**
- [x] Completed To-do
- [>] To-do saved for tomorrow

**Subsection B**
- [/] To-do half-completed
- [-] To-do canceled
- [ ] To-do uncompleted (not started)
```

This script carries over all uncompleted To-dos when a new file is created using Templater:
```markdown
---
filename: 2023-09-01.md
---

## 📌 Today's Goals
 
**Subsection A**
- [ ] To-do saved for tomorrow

**Subsection B**
- [/] To-do half-completed
- [ ] To-do uncompleted (not started)
```

> [!NOTE]
> To fully use the custom checkboxes a custom theme like
> [Minimal's Checkboxes](https://minimal.guide/Block+types/Checklists)
> or [my snippet](https://github.com/v-rogg/obsidian-snippets/) is recommended

## Installation

**Requirements**
- Having [Templater](https://github.com/SilentVoid13/Templater) installed

### Set up Templater

1. Set `Enable User System Command Functions` to `True`
2. Create a User Function with `carryover_todos`:`<path_to_script>/carryover_todos "<% tp.file.folder(true) %>" "<% tp.file.title %>"`. **Replace `<path_to_script>`**
3. Add the user function to a template like `<% tp.user.carryover_todos_rust({"section_title": <section_name>}) %>`. **Replace `<section_name>`**

## Compilation

**Requirements**
- Having [Rust/Cargo](https://www.rust-lang.org/tools/install) installed locally

- Run `cargo build --release`
