use std::panic;

use ::plsfix::{ExplainedText, ExplanationStep, Normalization, TextFixerConfig};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyTextFixerConfig {
    pub unescape_html: Option<bool>,
    pub remove_terminal_escapes: bool,
    pub fix_encoding: bool,
    pub restore_byte_a0: bool,
    pub replace_lossy_sequences: bool,
    pub decode_inconsistent_utf8: bool,
    pub fix_c1_controls: bool,
    pub fix_latin_ligatures: bool,
    pub fix_character_width: bool,
    pub uncurl_quotes: bool,
    pub fix_line_breaks: bool,
    pub remove_control_chars: bool,
    pub normalization: Option<Normalization>,
    pub max_decode_length: i32,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExplanationStep {
    pub transformation: String,
}

#[pymethods]
impl PyExplanationStep {
    #[getter]
    fn transformation(&self) -> String {
        self.transformation.clone()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExplainedText {
    pub text: String,
    pub steps: Option<Vec<PyExplanationStep>>,
}

#[pymethods]
impl PyExplainedText {
    #[getter]
    fn text(&self) -> String {
        self.text.clone()
    }

    #[getter]
    fn steps(&self) -> Option<Vec<PyExplanationStep>> {
        self.steps.clone()
    }
}

impl From<PyTextFixerConfig> for TextFixerConfig {
    fn from(config: PyTextFixerConfig) -> Self {
        TextFixerConfig {
            unescape_html: config.unescape_html,
            remove_terminal_escapes: config.remove_terminal_escapes,
            fix_encoding: config.fix_encoding,
            restore_byte_a0: config.restore_byte_a0,
            replace_lossy_sequences: config.replace_lossy_sequences,
            decode_inconsistent_utf8: config.decode_inconsistent_utf8,
            fix_c1_controls: config.fix_c1_controls,
            fix_latin_ligatures: config.fix_latin_ligatures,
            fix_character_width: config.fix_character_width,
            uncurl_quotes: config.uncurl_quotes,
            fix_line_breaks: config.fix_line_breaks,
            remove_control_chars: config.remove_control_chars,
            normalization: config.normalization,
            max_decode_length: config.max_decode_length,
        }
    }
}

impl From<ExplanationStep> for PyExplanationStep {
    fn from(step: ExplanationStep) -> Self {
        PyExplanationStep {
            transformation: step.transformation,
        }
    }
}

impl From<ExplainedText> for PyExplainedText {
    fn from(text: ExplainedText) -> Self {
        PyExplainedText {
            text: text.text,
            steps: match text.steps {
                Some(steps) => Some(steps.into_iter().map(|step| step.into()).collect()),
                None => None,
            },
        }
    }
}

#[pyfunction]
#[pyo3(signature = (text, config=None))]
pub fn fix_text(text: &str, config: Option<PyTextFixerConfig>) -> String {
    let config = config.map(PyTextFixerConfig::into);
    let config_ref = config.as_ref();

    let result = panic::catch_unwind(|| ::plsfix::fix_text(text, config_ref));

    match result {
        Ok(result) => result,
        Err(_) => text.to_string(),
    }
}

#[pyfunction]
#[pyo3(signature = (text, explain, config=None))]
pub fn fix_and_explain(
    text: &str,
    explain: bool,
    config: Option<PyTextFixerConfig>,
) -> PyExplainedText {
    let config = config.map(PyTextFixerConfig::into);
    let config_ref = config.as_ref();

    let result = panic::catch_unwind(|| ::plsfix::fix_and_explain(text, explain, config_ref));

    match result {
        Ok(result) => result.into(),
        Err(_) => PyExplainedText {
            text: text.to_string(),
            steps: None,
        },
    }
}

#[pymodule]
fn plsfix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fix_text, m)?)?;
    m.add_function(wrap_pyfunction!(fix_and_explain, m)?)?;
    Ok(())
}
