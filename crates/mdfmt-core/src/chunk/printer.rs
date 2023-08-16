pub enum Chunk {
    Single(String),
    Double(String),
}

pub struct ChunkPrinter(Vec<Chunk>);

impl ChunkPrinter {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, v: Chunk) {
        self.0.push(v);
    }

    pub fn print(&self) -> String {
        let mut res = String::new();
        for v in self.0.iter() {
            match v {
                Chunk::Single(s) => {
                    res.push_str(s);
                    res.push('\n');
                },
                Chunk::Double(s) => {
                    res.push_str(s);
                    res.push('\n');
                    res.push('\n');
                },
            }
        }
        res.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn empty() {
        let printer = ChunkPrinter::new();
        assert_eq!(printer.print(), "");
    }

    #[test]
    fn sigle_and_double() {
        let mut printer = ChunkPrinter::new();
        printer.push(Chunk::Single("a".to_string()));
        printer.push(Chunk::Double("b".to_string()));
        printer.push(Chunk::Single("c".to_string()));
        printer.push(Chunk::Double("d".to_string()));
        assert_eq!(
            printer.print(),
            indoc! {"
                a
                b

                c
                d"}
        );
    }
}
