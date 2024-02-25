import argparse
import logging
import pathlib

from ubpkg import manifests


def run():
    logging.basicConfig(level=logging.INFO)
    parser = argparse.ArgumentParser()
    parser.add_argument("manifest", type=pathlib.Path)
    args = parser.parse_args()
    manifest = manifests.Manifest(args.manifest)
    manifest.install()


