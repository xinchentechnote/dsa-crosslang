
def longest_vowel_substring(value :str):
    vowels = set("aeiouAEIOU")
    max_substr = ""
    substr = []
    # 遍历O(n)
    for ch in value:
        # 判断逻辑O(1)
        if ch in vowels:
            substr.append(ch)
        else:
            if len(substr) > len(max_substr):
                max_substr = "".join(substr)
            substr.clear()
    if len(substr) > len(max_substr):
        max_substr = "".join(substr)
    return max_substr, len(max_substr)
        