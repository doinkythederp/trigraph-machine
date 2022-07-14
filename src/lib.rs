/// The suffix of a trigraph and what the entire thing should be replaced with.
///
/// The first member of each tuple is the trigraph as written, without the "??"
/// prefix. The second member is what the trigraph should be replaced with. For
/// example, a tuple of `('=', '#')` means that the trigraph "??=" should be
/// replaced with the character "#".
#[derive(Debug, PartialEq, Eq)]
pub struct Trigraph(pub char, pub char);

impl Trigraph {
    pub const LEN: usize = 3;
}

/// A list of every trigraph in the C programming language.
pub const C_TRIGRAPH_DATA: &[Trigraph] = &[
    Trigraph('=', '#'),
    Trigraph('/', '\\'),
    Trigraph('\'', '^'),
    Trigraph('(', '['),
    Trigraph(')', ']'),
    Trigraph('!', '|'),
    Trigraph('<', '{'),
    Trigraph('>', '}'),
    Trigraph('-', '~'),
];

/// Data representing a trigraph found in a string.
#[derive(Debug)]
pub struct TrigraphLocation<'a> {
    pub index: usize,
    pub trigraph: &'a Trigraph,
}

impl<'a> TrigraphLocation<'a> {
    #[must_use]
    pub fn find(string: impl Iterator<Item = char>, trigraph_data: &'a [Trigraph]) -> Vec<Self> {
        let mut trigraphs = Vec::new();
        let mut question_mark_count = 0;
        for (index, character) in string.enumerate() {
            if question_mark_count < 2 {
                match character {
                    '?' => question_mark_count += 1,
                    _ => question_mark_count = 0,
                }
                continue;
            }

            let trigraph = trigraph_data
                .iter()
                .find(|trigraph| trigraph.0 == character);

            match trigraph {
                Some(trigraph) => trigraphs.push(Self {
                    // Dunno why I have to add 1 here. But if I remove it, it stops working.
                    index: index - Trigraph::LEN + 1,
                    trigraph,
                }),
                None if character == '?' => {}
                None => question_mark_count = 0,
            }
        }

        trigraphs
    }
}

#[cfg(test)]
mod tests {
    use crate::Trigraph;

    use super::{TrigraphLocation, C_TRIGRAPH_DATA};

    #[test]
    pub fn should_detect_c_trigraphs() {
        const TRIGRAPH_STRING: &str = "
            int main() ??<
                // My ??/
                comment.
            ??>
        ";

        let trigraphs = TrigraphLocation::find(TRIGRAPH_STRING.chars(), C_TRIGRAPH_DATA);
        assert_eq!(trigraphs.len(), 3);
        assert_eq!(trigraphs[0].trigraph.0, '<');
        assert_eq!(trigraphs[1].trigraph.0, '/');
        assert_eq!(trigraphs[2].trigraph.0, '>');
    }

    #[test]
    pub fn should_detect_three_question_mark_trigraphs() {
        const TRIGRAPH_STRING: &str = "???";

        let trigraphs = TrigraphLocation::find(TRIGRAPH_STRING.chars(), &[Trigraph('?', '_')]);
        assert_eq!(trigraphs.len(), 1);
        assert_eq!(trigraphs[0].trigraph.0, '?');
    }

    #[test]
    pub fn should_fail_if_trigraph_interupted() {
        const TRIGRAPH_STRING: &str = "?? =";

        let trigraphs = TrigraphLocation::find(TRIGRAPH_STRING.chars(), C_TRIGRAPH_DATA);
        assert_eq!(
            trigraphs.len(),
            0,
            "trigraph falsely detected: {:?}",
            trigraphs
        );
    }
}
