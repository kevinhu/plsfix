import plsfix


def test_config_creation_and_usage():
    config = plsfix.PyTextFixerConfig(
        unescape_html=False,
        remove_terminal_escapes=False,
        fix_encoding=True,
        fix_character_width=False,
        uncurl_quotes=False,
        remove_control_chars=False,
        normalization=None,
        max_decode_length=500000,
    )

    text = "Hello 'world'... ＬＯＵＤ"
    result = plsfix.fix_text(text, config)
    assert result == "Hello 'world'... ＬＯＵＤ"


def test_normalization_options():
    text = "café"

    config_nfc = plsfix.PyTextFixerConfig(normalization=plsfix.PyNormalization.NFC)
    result_nfc = plsfix.fix_text(text, config_nfc)
    assert result_nfc == "café"

    config_nfd = plsfix.PyTextFixerConfig(normalization=plsfix.PyNormalization.NFD)
    result_nfd = plsfix.fix_text(text, config_nfd)
    assert result_nfd == "cafe\u0301"

    config_nfkc = plsfix.PyTextFixerConfig(normalization=plsfix.PyNormalization.NFKC)
    result_nfkc = plsfix.fix_text(text, config_nfkc)
    assert result_nfkc == "café"

    config_nfkd = plsfix.PyTextFixerConfig(normalization=plsfix.PyNormalization.NFKD)
    result_nfkd = plsfix.fix_text(text, config_nfkd)
    assert result_nfkd == "cafe\u0301"

    config_no_norm = plsfix.PyTextFixerConfig(normalization=None)
    result = plsfix.fix_text(text, config_no_norm)
    assert result == "café"


def test_fix_and_explain_with_config():
    config = plsfix.PyTextFixerConfig(fix_encoding=True, remove_control_chars=True)

    text = "Hello\x00World"
    result = plsfix.fix_and_explain(text, True, config)

    assert result.text == "HelloWorld"
    assert isinstance(result.steps, list)
    assert len(result.steps) > 0


def test_all_config_options():
    """Test that all config options can be set."""
    config = plsfix.PyTextFixerConfig(
        unescape_html=True,
        remove_terminal_escapes=True,
        fix_encoding=False,
        restore_byte_a0=False,
        replace_lossy_sequences=False,
        decode_inconsistent_utf8=False,
        fix_c1_controls=True,
        fix_latin_ligatures=True,
        fix_character_width=True,
        uncurl_quotes=False,
        fix_line_breaks=True,
        remove_control_chars=True,
        normalization=plsfix.PyNormalization.NFKC,
        max_decode_length=100000,
    )

    result = plsfix.fix_text("test", config)
    assert result == "test"
