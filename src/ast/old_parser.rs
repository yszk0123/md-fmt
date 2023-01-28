use std::{iter::Peekable, str::Lines};

use crate::ast::{Ast, Block, Inline, ListItem};

pub struct Parser<'a> {
    lines: Peekable<Lines<'a>>,
    blocks: Vec<Block>,
}

impl<'a> Parser<'a> {
    pub fn parse(content: &'a String) -> Ast {
        Self::new(content).parse_content()
    }

    fn new(content: &'a String) -> Self {
        Self {
            lines: content.lines().peekable(),
            blocks: vec![],
        }
    }

    fn parse_content(mut self) -> Ast {
        while let Some(block) = self.seek() {
            self.blocks.push(block);
            self.lines.next();
        }

        Ast {
            front_matter: None,
            blocks: self.blocks,
        }
    }

    fn seek(&mut self) -> Option<Block> {
        if self.lines.peek()?.is_empty() {
            return Some(Block::Empty);
        }

        self.heading()
            .or_else(|| self.code_block())
            .or_else(|| self.list_item())
            .or_else(|| self.paragraph())
    }

    fn heading(&mut self) -> Option<Block> {
        let line = self.lines.peek()?;
        for (i, c) in line.char_indices() {
            if i == 0 && c != '#' {
                return None;
            }
            if c == ' ' {
                return Some(Block::Heading {
                    level: i as u8,
                    inlines: vec![Inline::Text(line[i + 1..].to_owned())],
                });
            }
            if c != '#' {
                return None;
            }
        }
        None
    }

    fn list_item(&mut self) -> Option<Block> {
        let line = self.lines.peek()?;
        let tail = line.trim_start();
        if tail.starts_with("- ") {
            let inline = Inline::Text(tail[2..].to_owned());
            return Some(Block::ListItem(ListItem {
                indent: line.len() - tail.len(),
                order: None,
                inlines: vec![inline],
            }));
        }
        if let Some(pos) = tail.find(". ") {
            let inline = Inline::Text(tail[2..].to_owned());
            let order = tail[..pos].parse::<u8>().unwrap_or(0);
            return Some(Block::ListItem(ListItem {
                indent: line.len() - tail.len(),
                order: Some(order),
                inlines: vec![inline],
            }));
        }
        None
    }

    fn code_block(&mut self) -> Option<Block> {
        if *(self.lines.peek()?) != "```" {
            return None;
        }

        let mut lines = vec![];

        self.lines.next();
        while let Some(line) = self.lines.peek() {
            if *line == "```" {
                break;
            }
            lines.push(*line);
            self.lines.next();
        }

        Some(Block::CodeBlock {
            label: String::from(""),
            content: lines.join("\n"),
        })
    }

    fn paragraph(&mut self) -> Option<Block> {
        Some(Block::Paragraph(vec![Inline::Text(
            self.lines.peek()?.to_string(),
        )]))
    }
}
