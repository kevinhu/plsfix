"""
Test ftfy's fixes using the data in `test_cases.json`.

I collected many test cases by listening to the Twitter streaming API for
millions of tweets, picking out examples with high weirdness, and seeing what
ftfy decoded them to. There are some impressive things that can happen to text,
even in an ecosystem that is supposedly entirely UTF-8.

Some examples come from the Common Crawl (particularly those involving
Windows-1250 mojibake, which is more common on arbitrary Web pages than on
Twitter), and some examples marked as 'synthetic' are contrived to test
particular features of ftfy.

Each test case is a dictionary containing the following items:

- "label": a label that will identify the test case in nosetests
- "original": the text to be ftfy'd
- "fixed": what the result of ftfy.fix_text should be on this text

There are also two optional fields:

- "fixed-encoding": what the result of just ftfy.fix_encoding should be.
  If missing, it will be considered to be the same as "fixed".
- "comment": possibly-enlightening commentary on the test case.
"""

import json
from pathlib import Path

import pytest

import plsfix

THIS_DIR = Path(__file__).parent
TEST_CASE_DIR = THIS_DIR / "test-cases"


def load_test_data() -> list[dict]:
    test_data = []
    for filepath in TEST_CASE_DIR.glob("*.json"):
        test_data.extend(json.load(filepath.open()))
    return test_data


TEST_DATA = load_test_data()

TESTS_THAT_PASS = [test for test in TEST_DATA if test["expect"] == "pass"]
TESTS_THAT_FAIL = [test for test in TEST_DATA if test["expect"] == "fail"]


@pytest.mark.parametrize("test_case", TEST_DATA)
def test_well_formed_example(test_case):
    assert test_case["expect"] in ("pass", "fail")




