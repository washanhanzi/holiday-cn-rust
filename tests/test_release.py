import importlib.util
import unittest
from datetime import datetime, timezone
from pathlib import Path

spec = importlib.util.spec_from_file_location("version", Path(__file__).resolve().parents[1] / "scripts/bump-version.py")
version = importlib.util.module_from_spec(spec)
spec.loader.exec_module(version)


class ReleaseVersionTests(unittest.TestCase):
    def test_timestamp_includes_seconds(self):
        now = datetime(2026, 9, 15, 12, 34, 56, tzinfo=timezone.utc)
        self.assertEqual(version.next_version("0.1.20250817", now), "0.2.20260915123456")

    def test_repeated_release_and_clock_skew(self):
        now = datetime(2026, 9, 15, 12, 34, 56, tzinfo=timezone.utc)
        self.assertEqual(version.next_version("0.2.20260915123456", now), "0.2.20260915123457")
        self.assertEqual(version.next_version("0.2.20260915235959", now), "0.2.20260916000000")


class SubmoduleUpdateTests(unittest.TestCase):
    def test_json_changes_inside_gitlink_and_non_data_updates(self):
        import os
        import subprocess
        import tempfile
        script = Path(__file__).resolve().parents[1] / "scripts/update-data.sh"
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            upstream, parent = root / "upstream", root / "parent"
            env = dict(os.environ, GIT_ALLOW_PROTOCOL="file", GIT_AUTHOR_NAME="Test", GIT_AUTHOR_EMAIL="test@example.com", GIT_COMMITTER_NAME="Test", GIT_COMMITTER_EMAIL="test@example.com")
            def git(cwd, *args):
                return subprocess.check_output(["git", "-c", "commit.gpgsign=false", "-C", str(cwd), *args], env=env, stderr=subprocess.PIPE, text=True).strip()
            upstream.mkdir()
            parent.mkdir()
            git(upstream, "init")
            (upstream / "2026.json").write_text("{}")
            git(upstream, "add", ".")
            git(upstream, "commit", "-m", "initial")
            git(parent, "init")
            git(parent, "submodule", "add", str(upstream), "holiday-cn")
            git(parent, "commit", "-am", "initial")
            original = git(parent / "holiday-cn", "rev-parse", "HEAD")
            def run_update():
                output = root / "output"
                output.write_text("")
                subprocess.run(["bash", str(script)], cwd=parent, env=dict(env, GITHUB_OUTPUT=str(output)), check=True, capture_output=True)
                return output.read_text().strip()
            self.assertEqual(run_update(), "changed=false")
            (upstream / "README.md").write_text("documentation")
            git(upstream, "add", ".")
            git(upstream, "commit", "-m", "docs")
            self.assertEqual(run_update(), "changed=false")
            self.assertEqual(git(parent / "holiday-cn", "rev-parse", "HEAD"), original)
            (upstream / "2026.json").write_text('{"days": []}')
            git(upstream, "commit", "-am", "data")
            self.assertEqual(run_update(), "changed=true")
            self.assertEqual(git(parent / "holiday-cn", "rev-parse", "HEAD"), git(upstream, "rev-parse", "HEAD"))
