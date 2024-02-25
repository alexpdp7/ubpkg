import logging
import pathlib
import yaml

from ubpkg import github


logger = logging.getLogger(__name__)


class Manifest:
    def __init__(self, path: pathlib.Path):
        self.yaml = yaml.load(path.read_text(), yaml.SafeLoader)

    def install(self):
        if "github" in self.yaml:
            url = github.get_url(self.yaml["github"]["repo"], matcher(self.yaml["github"]["asset"]))
        else:
            assert False, f"I do not know how to install {self.yaml}"
        logger.info("Downloading from %s", url)


def matcher(asset_name: str):
    def _match(candidate: str):
        return candidate == asset_name.format(os="linux", arch="amd64")
    return _match
