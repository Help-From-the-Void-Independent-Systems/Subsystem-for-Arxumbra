import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
FIXTURE_DIR = ROOT / "scripts" / "fixtures"

RUST_FORBIDDEN_WORDS = (
    "use",
    "let",
    "mut",
    "const",
    "static",
    "unsafe",
    "impl",
    "extern",
)

RUST_FORBIDDEN_PATTERNS = (
    (re.compile(r"&\s*mut\b"), "explicit mutable reference"),
    (re.compile(r"\*\s*(?:const|mut)\b"), "raw pointer type"),
    (re.compile(r"#\s*\[\s*doc\b"), "doc attribute"),
    (re.compile(r"\binclude\s*!\s*\("), "include! macro"),
    (re.compile(r"\binclude_str\s*!\s*\("), "include_str! macro"),
    (re.compile(r"\binclude_bytes\s*!\s*\("), "include_bytes! macro"),
    (re.compile(r"\bmacro_rules\s*!"), "macro_rules definition"),
    (re.compile(r"\b(?:unwrap|expect)\s*\("), "panicking accessor"),
    (re.compile(r"\b(?:panic|todo|unimplemented|unreachable)\s*!"), "panicking macro"),
)

LIFETIME_PATTERN = re.compile(r"'[a-z_][a-z_0-9]*")


def blank_out(text, start, end, out):
    for position in range(start, end):
        out.append(text[position] if text[position] == "\n" else " ")


def strip_rust(text):
    out = []
    index = 0
    length = len(text)
    while index < length:
        char = text[index]
        nxt = text[index + 1] if index + 1 < length else ""
        if char == "/" and nxt == "/":
            while index < length and text[index] != "\n":
                index += 1
            continue
        if char == "/" and nxt == "*":
            depth = 1
            index += 2
            while index < length and depth > 0:
                if text.startswith("/*", index):
                    depth += 1
                    index += 2
                elif text.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    if text[index] == "\n":
                        out.append("\n")
                    index += 1
            continue
        if char == '"':
            raw_hashes = 0
            prefix_end = index
            while prefix_end > 0 and text[prefix_end - 1] == "#":
                raw_hashes += 1
                prefix_end -= 1
            is_raw = (
                raw_hashes > 0
                and prefix_end > 0
                and text[prefix_end - 1] in "rb"
                and (
                    prefix_end - 2 < 0
                    or not (text[prefix_end - 2].isalnum() or text[prefix_end - 2] == "_")
                )
            )
            if is_raw:
                terminator = '"' + "#" * raw_hashes
                end = text.find(terminator, index + raw_hashes + 1)
                end = length if end < 0 else end + len(terminator)
                blank_out(text, index, end, out)
                index = end
                continue
            index += 1
            while index < length:
                piece = text[index]
                if piece == "\\":
                    out.append("  ")
                    index += 2
                    continue
                if piece == '"':
                    index += 1
                    break
                out.append(piece if piece == "\n" else " ")
                index += 1
            continue
        if char == "'":
            if text.startswith("'\\", index):
                end = index + 2
                if end < length and text[end] == '"':
                    end += 1
                if end < length and text[end] == "'":
                    blank_out(text, index, end + 1, out)
                    index = end + 1
                    continue
            if index + 2 < length and text[index + 2] == "'":
                out.append("   ")
                index += 3
                continue
            match = LIFETIME_PATTERN.match(text, index)
            if match:
                blank_out(text, index, match.end(), out)
                index = match.end()
                continue
        out.append(char)
        index += 1
    return "".join(out)


def strip_shell(text):
    out = []
    comment_lines = []
    index = 0
    length = len(text)
    brace_depth = 0
    while index < length:
        char = text[index]
        nxt = text[index + 1] if index + 1 < length else ""
        if char == "$" and nxt == "{":
            brace_depth += 1
            out.append("  ")
            index += 2
            continue
        if char == "}" and brace_depth > 0:
            brace_depth -= 1
            index += 1
            continue
        if char in ('"', "'"):
            quote = char
            index += 1
            while index < length and text[index] != quote:
                if quote == '"' and text[index] == "\\":
                    out.append("  ")
                    index += 2
                    continue
                if text[index] == "\n":
                    out.append("\n")
                index += 1
            out.append("  ")
            index += 1
            continue
        if char == "#" and brace_depth == 0:
            previous = text[index - 1] if index > 0 else "\n"
            at_word_start = index == 0 or previous in ("\n", " ", "\t", ";", "(")
            if at_word_start:
                comment_lines.append(text.count("\n", 0, index) + 1)
                while index < length and text[index] != "\n":
                    index += 1
                continue
        out.append(char)
        index += 1
    return "".join(out), comment_lines


def strip_hash_language(text):
    out = []
    index = 0
    length = len(text)
    while index < length:
        char = text[index]
        if char in ('"', "'"):
            quote = char
            triple = text.startswith(quote * 3, index)
            skip = quote * 3 if triple else quote
            index += len(skip)
            while index < length:
                if text[index] == "\\" and quote == '"':
                    out.append("  ")
                    index += 2
                    continue
                if text.startswith(skip, index):
                    index += len(skip)
                    break
                if text[index] == "\n":
                    out.append("\n")
                index += 1
            continue
        out.append(char)
        index += 1
    return "".join(out)


def check_rust_source(text, label):
    findings = []
    stripped = LIFETIME_PATTERN.sub(lambda m: " " * len(m.group(0)), strip_rust(text))
    for word in RUST_FORBIDDEN_WORDS:
        for match in re.finditer(r"\b%s\b" % word, stripped):
            if word == "mut" and is_approved_parameter_mut(stripped, match):
                continue
            line = stripped.count("\n", 0, match.start()) + 1
            findings.append((label, line, "forbidden construct: %s" % word))
    for pattern, reason in RUST_FORBIDDEN_PATTERNS:
        for match in pattern.finditer(stripped):
            line = stripped.count("\n", 0, match.start()) + 1
            findings.append((label, line, reason))
    fn_matches = list(re.finditer(r"\bfn\b", stripped))
    if len(fn_matches) != 1:
        findings.append((label, 0, "expected exactly one fn item, found %d" % len(fn_matches)))
    elif not re.match(r"\bfn\s+main\s*\(", stripped[fn_matches[0].start() :][:12]):
        findings.append((label, 0, "the single fn item is not fn main"))
    return findings


def is_approved_parameter_mut(stripped, match):
    preceding = stripped[: match.start()]
    preceding_token = re.search(r"([A-Za-z_0-9&*]+|\S)\s*$", preceding)
    if preceding_token is not None and preceding_token.group(1) in ("let", "ref", "&", "*"):
        return False
    return re.match(r"\bmut\s+[a-z_][a-z_0-9]*\s*:", stripped[match.start() :]) is not None


def check_hash_comments(text, label):
    findings = []
    stripped = strip_hash_language(text)
    for line_number, line in enumerate(stripped.splitlines(), 1):
        if line.find("#") >= 0:
            findings.append((label, line_number, "comment token in hash-language file"))
            break
    return findings


def check_shell_comments(text, label):
    comment_lines = strip_shell(text)[1]
    if comment_lines and comment_lines != [1]:
        return [(label, comment_lines[0], "comment token in shell file")]
    return []


def is_excluded(path):
    parts = set(path.parts)
    if ".git" in parts or "target" in parts or "docs" in parts:
        return True
    if FIXTURE_DIR in path.parents or path == FIXTURE_DIR:
        return True
    return path.name == "Cargo.lock"


def maintained_rust_files():
    return sorted((ROOT / "src").rglob("*.rs"))


def maintained_script_files():
    result = []
    for pattern in ("*.toml", "*.yml", "*.yaml", "*.py"):
        result.extend(path for path in ROOT.rglob(pattern) if not is_excluded(path))
    result.extend((ROOT / ".github" / "workflows").glob("*.yml"))
    result.extend((ROOT / ".github" / "workflows").glob("*.yaml"))
    result.extend((ROOT / "scripts").rglob("*.sh"))
    return sorted(set(path for path in result if not is_excluded(path) and path.is_file()))


def scan_tree():
    findings = []
    for path in maintained_rust_files():
        findings.extend(check_rust_source(path.read_text(), str(path.relative_to(ROOT))))
    for path in maintained_script_files():
        text = path.read_text()
        label = str(path.relative_to(ROOT))
        if path.suffix == ".sh":
            findings.extend(check_shell_comments(text, label))
        else:
            findings.extend(check_hash_comments(text, label))
    lock = ROOT / "Cargo.lock"
    if lock.exists() and any(
        line.lstrip().startswith("#") for line in lock.read_text().splitlines()
    ):
        findings.append(("Cargo.lock", 0, "comment lines in lockfile"))
    return findings


def fixture_findings(path):
    text = path.read_text()
    name = path.name
    if name.endswith(".rs.fixture"):
        return check_rust_source(text, name)
    if name.endswith(".yml.fixture") or name.endswith(".yaml.fixture"):
        return check_hash_comments(text, name)
    if name.endswith(".sh.fixture"):
        return check_shell_comments(text, name)
    return [(name, 0, "unhandled fixture kind")]


def selftest():
    findings = []
    fixtures = sorted(FIXTURE_DIR.glob("*.fixture"))
    if not fixtures:
        return [("fixtures", 0, "no fixtures found")]
    for path in fixtures:
        actual = fixture_findings(path)
        expect_clean = path.stem.startswith("negative")
        clean = not actual
        if expect_clean != clean:
            findings.append(
                (
                    path.name,
                    0,
                    "selftest mismatch: expected %s, scanner said %s (%s)"
                    % (
                        "clean" if expect_clean else "violations",
                        "clean" if clean else "violations",
                        actual[:3],
                    ),
                )
            )
    return findings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args()
    findings = selftest() if args.selftest else scan_tree() + selftest()
    for label, line, reason in findings:
        print("violation %s:%s %s" % (label, line, reason))
    if findings:
        print("policy scan FAILED with %d finding(s)" % len(findings))
        return 1
    print("policy scan passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
