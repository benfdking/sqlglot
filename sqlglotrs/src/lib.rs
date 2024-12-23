use pyo3::prelude::*;

pub mod benchmark;
pub mod settings;
pub mod tokenizer;
pub mod trie;
pub mod token;



// #[pymethods]
// impl Token {
//     #[pyo3(name = "__repr__")]
//     fn python_repr(&self) -> PyResult<String> {
//         Python::with_gil(|py| {
//             Ok(format!(
//                 "<Token token_type: {}, text: {}, line: {}, col: {}, start: {}, end: {}, comments: {}>",
//                 self.token_type_py.bind(py).repr()?,
//                 self.text.bind(py).repr()?,
//                 self.line,
//                 self.col,
//                 self.start,
//                 self.end,
//                 self.comments.bind(py).repr()?,
//             ))
//         })
//     }
// }

// #[pymodule]
// fn sqlglotrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<Token>()?;
//     m.add_class::<TokenTypeSettings>()?;
//     m.add_class::<TokenizerSettings>()?;
//     m.add_class::<TokenizerDialectSettings>()?;
//     m.add_class::<Tokenizer>()?;
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::{TokenTypeSettings, Tokenizer, TokenizerDialectSettings, TokenizerSettings};
//
//     #[test]
//     fn benchmark() {
//         // Read tokenizer settings
//         let settings_file = std::fs::read_to_string(
//             "/Users/benjaminking/Developer/sqlglot/.hacking/tokenizer_settings.json",
//         )
//         .unwrap();
//         let tokenizer_settings = serde_json::from_str::<TokenizerSettings>(&settings_file).unwrap();
//
//         let settings_type_file = std::fs::read_to_string(
//             "/Users/benjaminking/Developer/sqlglot/.hacking/token_type_settings.json",
//         )
//         .unwrap();
//         let settings_type_file =
//             serde_json::from_str::<TokenTypeSettings>(&settings_type_file).unwrap();
//
//         let dialect_settings = std::fs::read_to_string(
//             "/Users/benjaminking/Developer/sqlglot/.hacking/dialect_settings.json",
//         )
//         .unwrap();
//         let dialect_settings =
//             serde_json::from_str::<TokenizerDialectSettings>(&dialect_settings).unwrap();
//
//         let tokenizer = Tokenizer::new(tokenizer_settings, settings_type_file);
//
//         tokenizer.tokenize(crate::benchmark::LONG, &dialect_settings);
//     }
// }
