import pytest
from p0001_longest_vowel_substring import longest_vowel_substring

@pytest.mark.parametrize("input_str, expected", [
    ("", ("", 0)),                       # 空字符串
    ("bcdfg", ("", 0)),                  # 没有元音
    ("aeiou", ("aeiou", 5)),             # 全是元音
    ("hello", ("e", 1)),                 # 单个元音
    ("hellooo", ("ooo", 3)),             # 结尾连续元音
    ("helloooaeiou", ("oooaeiou", 8)),   # 多段元音，最长在结尾
    ("aabcdeeioou", ("eeioou", 6)),      # 中间最长
    ("AEIOUxyzabc", ("AEIOU", 5)),       # 大写元音
])
def test_longest_vowel_substring(input_str, expected):
    assert longest_vowel_substring(input_str) == expected
    