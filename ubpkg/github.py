import logging

import github


logger = logging.getLogger(__name__)


def get_releases(gh_repo):
    logging.info("Getting releases from gh/{gh_repo}")
    releases = github.Github().get_repo(gh_repo).get_releases()
    return [(r.tag_name, r.created_at) for r in releases]


def get_asset_url(gh_repo, release, matcher):
    release = github.Github().get_repo(gh_repo).get_release(release)
    assets = [a for a in release.assets if matcher(a.name)]
    assert len(assets) == 1, f"len({assets}) != 1 when searching asset {name} in {gh_repo}-{release}"
    asset = assets[0]
    return asset.browser_download_url


def get_url(repo, matcher):
    releases = get_releases(repo)
    latest_releases = sorted(releases, key=lambda r: r[1], reverse=True)
    assert len(latest_releases) > 0, f"Could not find releases of {repo}"
    latest_release = latest_releases[0]
    return get_asset_url(repo, latest_release[0], matcher)
