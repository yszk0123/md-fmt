[![Rust](https://github.com/yszk0123/md-fmt/actions/workflows/rust.yaml/badge.svg)](https://github.com/yszk0123/md-fmt/actions/workflows/rust.yaml)

# md-fmt

Learn Rust by implementing a tiny Markdown formatter.

## Example
(WIP)

### CLI

`hello-world.md``
```md
# hello
world!
```

```bash
# Fromat
$ md-fmt hello-world.md --write
# JSON
$ md-fmt hello-world.md --json | jq '.body[] | select(.type == "Section").value.title'
"hello"
```

### Rust

```rs
use anyhow::Result;
use mdfmt_core::{
    format,
    model::{Block, Section},
    parse,
};

fn main() -> Result<()> {
    let note = mdfmt_core::parse("# foo")?;
    for block in note.body {
        match block {
            Block::Section(Section { title, .. }) => {
                println!("title: {title}", title);
            },
            _ => {},
        }
    }

    let formatted = format("# foo")?;
    println!("formatted: {}", formatted);
}
```

### TypeScript

```ts
import { format, parse } from 'mdfmt-js';

const note = parse('# Hello, world!');
for (const block of note.body) {
  switch (block.type) {
    case 'Section': {
      console.log(`title: ${block.title}`);
      break;
    }
    default: {
      break;
    }
  }
}

const formatted = format('# Hello, world!');
console.log(`formatted: ${formatted}`);
```
