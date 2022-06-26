# make-adventure

Personal static site generator tool for generating text adventure games.

## Building

To build, you'll need a [Rust](https://www.rust-lang.org/) installation, including Cargo.

Run `cargo build --release` to compile the executable. You can also use `cargo install` to install the executable to your system.

## Usage

You can do `make-adventure --help` to print basic usage information. The gist is that you need to provide a config TOML file which describes the entire adventure, and a template file for generating the HTML files themselves. See the Config and Template sections for more specific information.

After providing a config file, run something like `make-adventure path/to/config.toml output_directory/`, and output HTML files will be generated in the output directory.

## Configuration

The configuration file requires a `template` value, for the path to the template HTML file used (see Template section below). An optional `additional-files` list can be provided which allows additional files to be copied to the output directory on build.

Pages can be provided link in the example below for actual game content. Each page is written like:

```toml
# Starts a section describing a page `my-page-name`. This is an internal symbol and won't be shown
# to a user.
[page.my-page-name]
# The title of the page, used by the template.
title = "The title of the page."
# Text content of the page, used by the template.
paragraphs = [
    "Zero or more paragraphs for this page. Only one here, though",
]
# Section for all links in the page, which users use to move to other pages.
[page.my-page-name.links]
# Specifies a link with the text "THE USER-FACING TEXT" which takes you to the page identified by
# `the-page-to-go-to`.
"THE USER-FACING TEXT" = "the-page-to-go-to"

```

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

[pages.start]
title = "The start page"
paragraphs = [
    "This is the first page, indexed 1.",
    "You can put as many paragraphs of text as you want, of course.",
]
[pages.start.links]
"GO TO ANOTHER PAGE" = "second"

[pages.second]
title = "A second page"
paragraphs = ["Not much text on this page."]
[pages.second.links]
"GO BACK TO THE START" = "start"
"CONTINUE ONWARDS" = "final"

[pages.final]
title = "Final page"
paragraphs = ["Last page! No links here..."]
[pages.final.links]

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
