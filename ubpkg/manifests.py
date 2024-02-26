import importlib.resources
import logging
import pathlib
import shutil
import urllib.request
import yaml

from ubpkg import github


logger = logging.getLogger(__name__)


def load_manifest(s):
    path = pathlib.Path(s)
    if path.exists():
        assert path.name.endswith(".ubpkg.yaml"), f"{path} does not end in .ubpkg.yaml"
        return path.name.removesuffix(".ubpkg.yaml"), path.read_text()
    path = importlib.resources.files("ubpkg") / "repo" / f"{s}.ubpkg.yaml"
    if path.exists():
        return s, path.read_text()


class Manifest:
    def __init__(self, s):
        name, content = load_manifest(s)
        self.yaml = yaml.load(content, yaml.SafeLoader)
        self.name = name

    def install(self):
        if "github" in self.yaml:
            url = github.get_url(
                self.yaml["github"]["repo"], matcher(self.yaml["github"]["asset"])
            )
        else:
            assert False, f"I do not know how to install {self.yaml}"
        path = pathlib.Path.home() / ".local" / "bin" / self.name
        logger.info("Downloading from %s to %s", url, path)
        with urllib.request.urlopen(url) as resp:
            with open(path, "wb") as bin:
                shutil.copyfileobj(resp, bin)
        path.chmod(0o755)


def matcher(asset_name: str):
    def _match(candidate: str):
        return candidate == asset_name.format(os="linux", arch="amd64")

    return _match
