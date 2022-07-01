# make-adventure

Personal-use static site generator tool for generating text adventure games.

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
    <title>{{index}} - {{title}}</title>
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

Each line may either be blank, a comment, a directive or text.

Blank lines (containing only whitespace) and comments (starting with any amount of whitespace and then a hash (`#`)) are ignored, and have no effect on the surrounding lines.

### Directives

A directive is a line starting with a keyword, followed by an argument. Usable keywords are:

- `page`, for declaring pages.
- `title`, for setting the title of a page.
- `text`, for adding paragraphs of text to a page.
- `link`, for adding links between pages.

Any text which follows the keyword is considered its argument. For example:

```
page any-old-string
```

Would be a directive of kind `page`, with argument `any-old-string`.

To add additional information to a directive, further *indented* directives can follow it, so:

```
page My Page
    title
        The title of the page.
```

Declares a `page` directive (with argument `My Page`) to have a child directive of kind `title`. Which, in turn, has chlid *text*.

### Text lines

Any line which is not empty, a comment, or a directive, is considered a *text* line. These are used to actually provide the script with text.

### Example

```
page end-of-game
    title
        You are dead.

    text
        After falling through a trap door, you have landed in a trap and died.

        Game over.

    link start
        START OVER
```

This, for example, would declare a page identified by `end-of-game`, with a title "You are dead.", and with two paragraphs of text ("After falling through..." and "Game over."), and one link to a `start` page with link text "START OVER".

## To-dos

### Page-specific files

I need to be able to add images to my pages, so specifying custom files for each page will be required. The files will be copied into the output directory, and have their paths passed to the templating system.

### Graphical front-end

Unlikely to do this in the near future. Using a graphical interface will make managing complex stories easier, since ensuring links are correct in a text file can be difficult.

### Multiple files

I'd like to be able to split the config files into multiple parts, so that I'm not staring at a single file the whole time I'm writing. Specifying a 'root' config, and then recursively including others would be very useful.

### Documentation

No doubt I'm going to forget about this for a few months before coming back to it. Some documentation would be nice for future me. ;)

### Directive escapes

Text lines can't start with directives, which means its impossible to write a line of text like "page one", since it'll be parsed as a directive. There should be a way to escape directives so a line is always considered text.
