use std::io;

use trigraph::{Trigraph, TrigraphLocation, C_TRIGRAPH_DATA};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let line = line?;

        let trigraphs = TrigraphLocation::find(line.chars(), C_TRIGRAPH_DATA);
        for n in 0..trigraphs.len() {
            let trigraph_loc = trigraphs[n].index;
            let previous_loc = if n == 0 {
                0
            } else {
                trigraphs[n - 1].index + Trigraph::LEN
            };
            print!(
                "{}{}",
                &line[previous_loc..trigraph_loc],
                trigraphs[n].trigraph.1
            );
        }

        let last_trigraph_end = if let Some(trigraph) = trigraphs.last() {
            trigraph.index + Trigraph::LEN
        } else {
            0
        };

        println!("{}", &line[last_trigraph_end..]);
    }

    Ok(())
}
