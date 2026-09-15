"""Select monotonically increasing UTC timestamp patch versions."""
import re
import sys
import tomllib
from datetime import datetime, timedelta, timezone
from pathlib import Path


def next_version(current, now):
    stamp = now.astimezone(timezone.utc).strftime("%Y%m%d%H%M%S")
    if re.fullmatch(r"0\.2\.\d{14}", current):
        previous = datetime.strptime(current[4:], "%Y%m%d%H%M%S").replace(tzinfo=timezone.utc)
        stamp = max(stamp, (previous + timedelta(seconds=1)).strftime("%Y%m%d%H%M%S"))
    return f"0.2.{stamp}"


def main():
    path = Path("Cargo.toml")
    content = path.read_text()
    current = tomllib.loads(content)["package"]["version"]
    if "--check" in sys.argv:
        if not re.fullmatch(r"0\.2\.\d{14}", current):
            raise SystemExit("Expected version 0.2.YYYYMMDDHHMMSS")
        datetime.strptime(current[4:], "%Y%m%d%H%M%S")
        lock = tomllib.loads(Path("Cargo.lock").read_text())
        assert any(p["name"] == "holiday-cn" and p["version"] == current for p in lock["package"]), "Cargo.lock version mismatch"
        print(current)
    else:
        version = next_version(current, datetime.now(timezone.utc))
        path.write_text(content.replace(f'version = "{current}"', f'version = "{version}"', 1))
        print(version)


if __name__ == "__main__":
    main()
