import argparse
import logging

from ubpkg import manifests


def run():
    logging.basicConfig(level=logging.INFO)
    parser = argparse.ArgumentParser()
    parser.add_argument("manifest", help="path to a manifest or name of a package")
    args = parser.parse_args()
    manifest = manifests.Manifest(args.manifest)
    manifest.install()
