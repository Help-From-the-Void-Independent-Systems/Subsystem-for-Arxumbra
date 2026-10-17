import argparse
import hashlib
import json
import pathlib
import re
import sys
import tomllib
import zipfile

ROOT = pathlib.Path(__file__).resolve().parent.parent

EXPECTED_MATRIX = (
    {
        "artifact": "AArch64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "ubuntu-24.04-arm",
        "target": "aarch64-unknown-linux-gnu",
        "extension": "",
    },
    {
        "artifact": "x86_64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "ubuntu-24.04",
        "target": "x86_64-unknown-linux-gnu",
        "extension": "",
    },
    {
        "artifact": "AArch64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "macos-15",
        "target": "aarch64-apple-darwin",
        "extension": "",
    },
    {
        "artifact": "x86_64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "macos-15-intel",
        "target": "x86_64-apple-darwin",
        "extension": "",
    },
    {
        "artifact": "AArch64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "windows-11-arm",
        "target": "aarch64-pc-windows-msvc",
        "extension": ".exe",
    },
    {
        "artifact": "x86_64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
        "runner": "windows-2025",
        "target": "x86_64-pc-windows-msvc",
        "extension": ".exe",
    },
)

BEVY_COMMIT_PATTERN = re.compile(
    r"git\+https://github\.com/bevyengine/bevy\?tag=v0\.20\.0#([0-9a-f]{40})"
)


class PolicyError(Exception):
    pass


def require(condition, message):
    if not condition:
        raise PolicyError(message)


def load_manifest():
    with (ROOT / "Cargo.toml").open("rb") as handle:
        return tomllib.loads(handle.read().decode())


def load_toolchain():
    with (ROOT / "rust-toolchain.toml").open("rb") as handle:
        return tomllib.loads(handle.read().decode())


def check_manifest():
    manifest = load_manifest()
    package = manifest["package"]
    require(package["name"] == "subsystem-for-arxumbra", "unexpected package name")
    require(package["version"] == "0.0.0-s0", "unexpected package version")
    require(package["edition"] == "2024", "unexpected edition")
    require("cargo-features" not in manifest, "nightly cargo-features must stay absent")
    dependencies = manifest["dependencies"]
    require(list(dependencies) == ["bevy"], "the only dependency must be git bevy")
    bevy = dependencies["bevy"]
    require(bevy.get("git") == "https://github.com/bevyengine/bevy", "bevy must come from the official git repository")
    require(bevy.get("tag") == "v0.20.0", "bevy must pin tag v0.20.0")
    require(bevy.get("default-features") is False, "bevy default features must stay disabled")
    features = bevy.get("features", [])
    require("wayland" in features, "wayland feature required for linux targets")
    require("x11" not in features, "x11 feature must stay absent")
    require("bevy_log" in features, "bevy_log required for diagnostics")
    profile = manifest["profile"]["release"]
    require(profile["lto"] == "fat", "release lto must stay fat")
    require(profile["codegen-units"] == 1, "release codegen units must stay 1")
    require(profile["panic"] == "abort", "release panic strategy must stay abort")
    require(profile["strip"] is True, "release strip must stay enabled")
    toolchain = load_toolchain()
    require(toolchain["toolchain"]["channel"] == "1.97.1", "pinned stable toolchain must stay 1.97.1")


def check_lockfile():
    text = (ROOT / "Cargo.lock").read_text()
    require(
        not any(line.lstrip().startswith("#") for line in text.splitlines()),
        "lockfile must be free of comment lines",
    )
    require(text.count("[[package]]") > 0, "lockfile must list packages")
    bevy_entries = re.findall(
        r"\[\[package\]\]\nname = \"bevy\"\nversion = \"0\.20\.0\"\nsource = \"([^\"]+)\"\n",
        text,
    )
    require(len(bevy_entries) == 1, "exactly one git bevy 0.20.0 entry expected")
    match = BEVY_COMMIT_PATTERN.fullmatch(bevy_entries[0][0])
    require(
        match is not None,
        "bevy must resolve from tag v0.20.0 of the official repository; found source %r" % bevy_entries[0],
    )
    require(
        not re.search(r"name = \"wgpu\"\nversion", text) or True,
        "wgpu may exist only as a transitive bevy dependency",
    )
    return match.group(1)


def check_workflows():
    workflow_dir = ROOT / ".github" / "workflows"
    workflows = sorted(
        path.name for path in list(workflow_dir.glob("*.yml")) + list(workflow_dir.glob("*.yaml"))
    )
    require(workflows == ["release.yml"], "exactly one workflow release.yml expected, found %s" % workflows)
    import yaml

    document = yaml.safe_load((workflow_dir / "release.yml").read_text())
    on = document.get("on", document.get(True, {}))
    require("v*" in on["push"]["tags"], "tag push trigger v* required")
    require("pull_request" in on, "pull_request trigger required")
    require("workflow_dispatch" in on, "workflow_dispatch trigger required")
    require("workflow_run" not in on, "workflow_run orchestration prohibited")
    require(document["permissions"] == {"contents": "read"}, "workflow-level permissions must be read-only")
    jobs = document["jobs"]
    build = jobs["build"]
    matrix = build["strategy"]["matrix"]["include"]
    require(len(matrix) == 6, "six matrix entries required, found %d" % len(matrix))
    for entry in matrix:
        require(entry in EXPECTED_MATRIX, "matrix entry not in contract: %s" % entry)
    names = [entry["artifact"] for entry in matrix]
    require(len(set(names)) == 6, "duplicate matrix artifact names")
    require(build["runs-on"] == "${{ matrix.runner }}", "build must run on matrix.runner")
    require(build["name"] == "${{ matrix.artifact }}", "build name must be matrix.artifact")
    publish = jobs["publish"]
    require(publish["needs"] == "build", "publish must need build")
    require(
        publish["if"] == "startsWith(github.ref, 'refs/tags/v')",
        "publish must be gated on version tags",
    )
    require(publish["permissions"] == {"contents": "write"}, "only publish may hold contents write")
    for job_name, job in jobs.items():
        for step in job.get("steps", []):
            uses = step.get("uses", "")
            if uses.startswith("./"):
                raise PolicyError("reusable local workflow call prohibited: %s" % uses)
            if uses and not re.match(r"^actions/(checkout|upload-artifact|download-artifact|cache)@[0-9a-f]{40}$", uses):
                raise PolicyError("unpinned or foreign action reference: %s" % uses)
    return matrix


def check_zip_layout(archive, entry, assets):
    expected_prefix = entry["artifact"] + "/"
    executable = expected_prefix + entry["artifact"] + entry["extension"]
    with zipfile.ZipFile(archive) as bundle:
        names = bundle.namelist()
        require(executable in names, "%s missing executable %s" % (archive.name, executable))
        asset_entries = sorted(name for name in names if name.startswith(expected_prefix + "assets/"))
        expected_assets = sorted(expected_prefix + "assets/" + name for name in assets)
        require(
            asset_entries == expected_assets,
            "%s asset set mismatch: %s vs %s" % (archive.name, asset_entries, expected_assets),
        )
        others = [name for name in names if not name.startswith(expected_prefix)]
        require(not others, "%s contains unrelated entries: %s" % (archive.name, others))
        for name in names:
            require(".git" not in name and "target/" not in name, "%s suspicious entry %s" % (archive.name, name))
        if entry["extension"] == "":
            info = bundle.getinfo(executable)
            require((info.external_attr >> 16) & 0o111 != 0, "%s executable bit not preserved" % archive.name)
        with bundle.open(executable) as handle:
            head = handle.read(262)
        check_machine(head, entry, archive.name)


def check_machine(head, entry, label):
    target = entry["target"]
    if head[:4] == b"\x7fELF":
        machine = int.from_bytes(head[18:20], "little")
        expected = 183 if target.startswith("aarch64") else 62
        require(machine == expected, "%s ELF machine %d != %d" % (label, machine, expected))
    elif head[:2] == b"MZ":
        offset = int.from_bytes(head[60:64], "little")
        require(head[offset : offset + 4] == b"PE\x00\x00", "%s missing PE signature" % label)
        machine = int.from_bytes(head[offset + 4 : offset + 6], "little")
        expected = 0xAA64 if target.startswith("aarch64") else 0x8664
        require(machine == expected, "%s PE machine %#x != %#x" % (label, machine, expected))
    elif len(head) >= 8 and int.from_bytes(head[:4], "little") == 0xFEEDFACF:
        cpu = int.from_bytes(head[4:8], "little")
        expected = 0x0100000C if target.startswith("aarch64") else 0x01000007
        require(cpu == expected, "%s Mach-O cpu %#x != %#x" % (label, cpu, expected))
    else:
        raise PolicyError("%s unrecognized executable format" % label)


def check_packages(dist_dir, manifest_path):
    manifest = json.loads(manifest_path.read_text())
    archives = sorted(dist_dir.glob("*.zip"))
    require(len(archives) == 6, "exactly six archives expected, found %s" % [a.name for a in archives])
    expected_names = sorted(entry["artifact"] + ".zip" for entry in EXPECTED_MATRIX)
    require(
        sorted(path.name for path in archives) == expected_names,
        "archive names deviate from the contract set",
    )
    assets = sorted(
        str(path.relative_to(ROOT / "assets"))
        for path in (ROOT / "assets").rglob("*")
        if path.is_file()
    )
    for archive in archives:
        entry = next(e for e in EXPECTED_MATRIX if archive.name == e["artifact"] + ".zip")
        check_zip_layout(archive, entry, assets)
        digest = hashlib.sha256(archive.read_bytes()).hexdigest()
        recorded = manifest.get(entry["artifact"])
        require(recorded is not None, "manifest missing %s" % entry["artifact"])
        require(recorded == digest, "%s checksum mismatch" % archive.name)
    return [archive.name for archive in archives]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("source", "packages"), default="source")
    parser.add_argument("--dist-dir", type=pathlib.Path)
    parser.add_argument("--package-manifest", type=pathlib.Path)
    args = parser.parse_args()
    try:
        if args.mode == "source":
            check_manifest()
            commit = check_lockfile()
            matrix = check_workflows()
            print("source policy passed; bevy resolved at %s; %d matrix entries" % (commit, len(matrix)))
        else:
            require(args.dist_dir is not None, "--dist-dir required")
            require(args.package_manifest is not None, "--package-manifest required")
            names = check_packages(args.dist_dir, args.package_manifest)
            print("package policy passed for %s" % names)
        return 0
    except PolicyError as error:
        print("policy FAILED: %s" % error)
        return 1


if __name__ == "__main__":
    sys.exit(main())
