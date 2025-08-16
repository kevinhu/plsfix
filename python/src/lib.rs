use std::panic;

use ::plsfix::{ExplainedText, ExplanationStep, Normalization, TextFixerConfig};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum PyNormalization {
    NFC,
    NFKC,
    NFD,
    NFKD,
}

impl From<PyNormalization> for Normalization {
    fn from(norm: PyNormalization) -> Self {
        match norm {
            PyNormalization::NFC => Normalization::NFC,
            PyNormalization::NFKC => Normalization::NFKC,
            PyNormalization::NFD => Normalization::NFD,
            PyNormalization::NFKD => Normalization::NFKD,
        }
    }
}


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
    pub normalization: Option<PyNormalization>,
    pub max_decode_length: i32,
}

#[pymethods]
impl PyTextFixerConfig {
    #[new]
    #[pyo3(signature = (
        unescape_html=None,
        remove_terminal_escapes=true,
        fix_encoding=true,
        restore_byte_a0=true,
        replace_lossy_sequences=true,
        decode_inconsistent_utf8=true,
        fix_c1_controls=true,
        fix_latin_ligatures=true,
        fix_character_width=true,
        uncurl_quotes=true,
        fix_line_breaks=true,
        remove_control_chars=true,
        normalization=None,
        max_decode_length=1000000
    ))]
    pub fn new(
        unescape_html: Option<bool>,
        remove_terminal_escapes: bool,
        fix_encoding: bool,
        restore_byte_a0: bool,
        replace_lossy_sequences: bool,
        decode_inconsistent_utf8: bool,
        fix_c1_controls: bool,
        fix_latin_ligatures: bool,
        fix_character_width: bool,
        uncurl_quotes: bool,
        fix_line_breaks: bool,
        remove_control_chars: bool,
        normalization: Option<PyNormalization>,
        max_decode_length: i32,
    ) -> Self {
        PyTextFixerConfig {
            unescape_html,
            remove_terminal_escapes,
            fix_encoding,
            restore_byte_a0,
            replace_lossy_sequences,
            decode_inconsistent_utf8,
            fix_c1_controls,
            fix_latin_ligatures,
            fix_character_width,
            uncurl_quotes,
            fix_line_breaks,
            remove_control_chars,
            normalization,
            max_decode_length,
        }
    }
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
            normalization: config.normalization.map(|n| n.into()),
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
fn plsfix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fix_text, m)?)?;
    m.add_function(wrap_pyfunction!(fix_and_explain, m)?)?;
    m.add_class::<PyTextFixerConfig>()?;
    m.add_class::<PyExplainedText>()?;
    m.add_class::<PyExplanationStep>()?;
    m.add_class::<PyNormalization>()?;
    Ok(())
}
