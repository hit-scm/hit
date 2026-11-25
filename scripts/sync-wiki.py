"""Publish the repository's Markdown pages into an existing wiki checkout."""

import re
import sys
from pathlib import Path


PAGE_NAME = re.compile(r"[A-Za-z][A-Za-z0-9-]*\.md")
LOCAL_LINK = re.compile(r"(\]\()([A-Za-z][A-Za-z0-9-]*\.md)(?=[#)])")


def sync(source: Path, wiki: Path) -> None:
    pages = {
        page.name: page
        for page in sorted(source.glob("*.md"))
        if page.is_file() and PAGE_NAME.fullmatch(page.name)
    }
    if "Home.md" not in pages:
        raise ValueError("The documentation must include Home.md.")

    manifest = wiki / ".hit-docs-pages"
    previously_published = (
        set(manifest.read_text(encoding="utf-8").splitlines())
        if manifest.exists()
        else set()
    )

    def wiki_link(match: re.Match) -> str:
        name = match[2]
        if name not in pages:
            raise ValueError(f"Unknown documentation page: {name}")
        return match[1] + pages[name].stem

    rendered = {
        name: LOCAL_LINK.sub(wiki_link, page.read_text(encoding="utf-8"))
        for name, page in pages.items()
    }
    for name, content in rendered.items():
        (wiki / name).write_text(content, encoding="utf-8")
    for name in previously_published - pages.keys():
        if PAGE_NAME.fullmatch(name):
            (wiki / name).unlink(missing_ok=True)

    manifest.write_text("".join(f"{name}\n" for name in pages), encoding="utf-8")


if __name__ == "__main__":
    sync(Path(sys.argv[1]), Path(sys.argv[2]))
