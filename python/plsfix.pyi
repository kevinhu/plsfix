from typing import Optional, List

class PyTextFixerConfig:
    unescape_html: Optional[bool]
    remove_terminal_escapes: bool
    fix_encoding: bool
    restore_byte_a0: bool
    replace_lossy_sequences: bool
    decode_inconsistent_utf8: bool
    fix_c1_controls: bool
    fix_latin_ligatures: bool
    fix_character_width: bool
    uncurl_quotes: bool
    fix_line_breaks: bool
    remove_control_chars: bool
    normalization: Optional[str]
    max_decode_length: int

class PyExplanationStep:
    transformation: str

class PyExplainedText:
    text: str
    steps: Optional[List[PyExplanationStep]]

def fix_text(text: str, config: Optional[PyTextFixerConfig] = None) -> str: ...
def fix_and_explain(
    text: str, explain: bool, config: Optional[PyTextFixerConfig] = None
) -> PyExplainedText: ...
