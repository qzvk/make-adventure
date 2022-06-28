# make-adventure

Personal static site generator tool for generating text adventure games.

## Table of Contents

1. [Building](#building)
2. [Usage](#usage)
1. [Configuration](#configuration)
2. [Template](#template)
3. [Script](#script)
4. [To-dos](#to-dos)

## Building

To build, you'll need a [Rust](https://www.rust-lang.org/) installation, including Cargo.

Run `cargo build --release` to compile the executable. You can also use `cargo install` to install the executable to your system.

## Usage

You can do `make-adventure --help` to print basic usage information. You must provide three things to generate an adventure using this tool:

- A config file, used to configure the adventure.
- A script file, which contains user-facing content.
- A template file, used to generate HTML files for each page.

After providing these, run something like `make-adventure path/to/config.toml output_directory/`, and output HTML files will be generated in the output directory.

## Configuration

Make-adventure uses a TOML config file, which takes the following keys:

- `template`, the path to the template file.
- `script`, the path to the script file.
- `additional-files` (optional), a list of additional files to copy after a build.

### Example config

```toml
# The path to the template file for each page.
template = "template.html"

# Additional files to copy into the output directory after a successful build.
additional-files = [
    "index.html",
    "style.css",
    "main.js",
]

# Path of the script file.
script = "script.txt"
```

## Template

This uses [Handlebars](https://lib.rs/crates/handlebars) as a templating system, so handlebars syntax is used within template files.

Provided variables are:

- `title` - the title of the page, as specified in the config.
- `paragraphs` - a list of strings of each paragraph of the page.
- `links` - a list of integer-string pairs, generated from configured page links.
  - `text` - The text of the link, to be shown to the user.
  - `index` - The index of the page which is linked to. This will also be the name of the generated file, so linking to a file can be done link `<a href="{{index}}.html">{{text}}</a>`.

### Example template

```html
<!DOCTYPE html>

<html lang="en">
<head>
    <meta charset="utf-8" />
    <title>{{title}}</title>
</head>
<body>
    <h1>{{title}}</h1>

    {{#each paragraphs}}
    <p>{{this}}</p>
    {{/each}}

    {{#each links}}
    <a href="{{index}}.html">{{text}}</a>
    {{/each}}
</body>
</html>
```

## Script

The script file contains all of the user-facing content for an adventure, and uses a simple Python-like language to describe each page, and links between them.

I haven't written the parser for it yet, and until then, I'll leave this section incomplete.

**TODO**

## To-dos

### Deterministic page numbers

Since serde is used for configuration parsing, and each page gets stored in a `HashMap`, the outputted page numbers are not consistent across builds. This is not desirable, since updates which only append or modify pages (and not re-arrange any) should not modify pre-existing page numbers. This makes 'saving your place' impossible, and also looks weird to a user, if they go from page 1 to 15 in one step, for example.

### Better configuration language

Currently, TOML is being used, which creates a lot of visual noise and doesn't feel like the correct choice. A different language should be used for writing and configuring pages. This will likely either be Markdown or a simple custom language.

### Page-specific files

I need to be able to add images to my pages, so specifying custom files for each page will be required. The files will be copied into the output directory, and have their paths passed to the templating system.

### Graphical front-end

Unlikely to do this in the near future. Using a graphical interface will make managing complex storied easier, since ensuring links are correct in a text file is difficult and error prone.

### Multiple files

I'd like to be able to split the config files into multiple parts, so that I'm not staring at a single file the whole time I'm writing. Specifying a 'root' config, and then recursively including others would be very useful.
