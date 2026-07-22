#!/usr/bin/env python3
"""Lossless lexical tokens for strace text records."""

from __future__ import annotations

import re
from dataclasses import asdict, dataclass
from typing import Final

TOKEN_PATTERN: Final[re.Pattern[str]] = re.compile(
    r'(?P<whitespace>\s+)|(?P<string>"(?:\\.|[^"\\])*")|'
    r'(?P<number>-?(?:\d+\.\d+|\d+)(?:[eE][+-]?\d+)?)|'
    r'(?P<identifier>[A-Za-z_][A-Za-z0-9_]*)|'
    r'(?P<operator>\+\+\+|---|\.\.\.|->|=>|==|!=|<=|>=)|(?P<symbol>.)',
    re.DOTALL,
)


@dataclass(frozen=True)
class Token:
    """One ordered token whose text occupies the half-open source span."""

    kind: str
    text: str
    start: int
    end: int

    def json(self) -> dict[str, str | int]:
        """Return the JSON-compatible token representation."""
        return asdict(self)


def lex(text: str) -> list[Token]:
    """Tokenize all characters and fail if reconstruction is not exact."""
    tokens = [Token(match.lastgroup or "symbol", match.group(), *match.span()) for match in TOKEN_PATTERN.finditer(text)]
    if "".join(token.text for token in tokens) != text:
        raise ValueError("lexer did not preserve the complete source record")
    return tokens
