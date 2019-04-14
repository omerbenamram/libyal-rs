import os
import shlex
import toml
import subprocess
import argparse
import re

parser = argparse.ArgumentParser("libyal-release")

SEMVER = re.compile(r"(\d)\.(\d).(\d)")


class SemVer:
    major: int
    minor: int
    patch: int

    def __init__(self, major, minor, patch):
        self.major = major
        self.minor = minor
        self.patch = patch

    def clone(self) -> 'SemVer':
        return SemVer(self.major, self.minor, self.patch)

    @staticmethod
    def from_string(s: str) -> 'SemVer':
        major, minor, patch = SEMVER.match(s).groups()
        return SemVer(int(major), int(minor), int(patch))

    def increment_patch(self):
        new = self.clone()
        new.patch += 1
        return new

    def to_string(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"


LIBYAL_LIBRARIES_DIRECTORIES = ["common", "common-build", "libcerror-sys", "libbfio-sys", "libbfio", "libfsntfs-sys",
                                "libfsntfs"]
LIBYAL_LIBRARIES_PACKAGES = ["libyal-rs-common", "libyal-rs-common-build", "libcerror-sys", "libbfio-sys",
                             "libfsntfs-sys", "libbfio-rs", "libfsntfs-rs"]


def main():
    for d in LIBYAL_LIBRARIES_DIRECTORIES:
        new_config = None

        with open(os.path.join(d, "Cargo.toml"), "r") as t:
            config = toml.load(t)
            current_version = SemVer.from_string(config["package"]["version"])
            new_version = current_version.increment_patch().to_string()
            package_name = config["package"]["name"]

            print(f"Current version of: {package_name} is {current_version}, updating to {new_version}")

            config["package"]["version"] = new_version

            # dependencies are updated in order in `LIBYAL_LIBRARIES_DIRECTORIES`,
            # which ensures the dependecy was updated and uploaded beforehand
            for dependency in config["dependencies"]:
                if dependency in LIBYAL_LIBRARIES_PACKAGES:
                    config["dependencies"][dependency]["version"] = new_version

            new_config = config

        assert new_config is not None

        with open(os.path.join(d, "Cargo.toml"), "w") as t:
            toml.dump(new_config, t)

        s = subprocess.run(shlex.split("cargo release --no-dev-version --skip-tag --skip-push"), cwd=d)
        s.check_returncode()

    print("Pushing")
    subprocess.run(shlex.split("git push"))


if __name__ == "__main__":
    main()
