# Malformed: Broken Arrows
# Tests arrows with broken connections

┌──┐    ┌──┐
│A │───│B │  # Normal arrow
└──┘    └──┘

Broken arrow - missing connection:
┌──┐    ┌──┐
│A │    │B │
└──┘ ── └──┘  # Gap in arrow

Arrow with invalid characters:
┌──┐    ┌──┐
│A │→→→│B │  # Wrong arrow characters
└──┘    └──┘

Vertical arrow issues:
┌──┐
│A │
│  │  # Missing arrow body
│  │
└──┘
┌──┐
│B │
└──┘