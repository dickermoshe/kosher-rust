from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ExtractedMethod:
    name: str
    qualified_name: str
    package: str
    class_name: str
    qualified_class: str
    file: str
    line: int
    return_type: str
    parameters: list[str]
    annotations: list[str]
    is_deprecated: bool
    docs: str


@dataclass(frozen=True)
class UserDocs:
    meaning: str
    calculation: str
    notes: list[str]
    deprecated_note: str


@dataclass(frozen=True)
class DocumentedMethod:
    name: str
    qualified_name: str
    package: str
    class_name: str
    qualified_class: str
    file: str
    line: int
    return_type: str
    parameters: list[str]
    annotations: list[str]
    is_deprecated: bool
    docs: str
    user_docs: UserDocs
