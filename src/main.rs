mod lab1;

use lab1::{create_tiny_lexical_dfa, reset_global_tables};

fn main() {
    reset_global_tables();
    let dfa = create_tiny_lexical_dfa();
    let sample = "read x;\n42";
    let tokens = dfa.long_text_search(sample);
    for token in tokens {
        println!("{:?}", token);
    }
}
