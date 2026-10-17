import argparse
import hashlib
import pathlib
import zipfile


def collect(source):
    for path in sorted(source.rglob("*")):
        relative = path.relative_to(source.parent)
        yield relative, None if path.is_dir() else path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=pathlib.Path)
    parser.add_argument("archive", type=pathlib.Path)
    args = parser.parse_args()
    archive = args.archive
    if archive.suffix != ".zip":
        archive = archive.with_name(archive.name + ".zip")
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
        for relative, path in collect(args.source):
            is_directory = path is None
            is_executable = path is not None and path.suffix == ""
            info = zipfile.ZipInfo(str(relative) + ("/" if is_directory else ""))
            mode = 0o755 if is_directory or is_executable else 0o644
            info.external_attr = mode << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            bundle.writestr(info, b"" if is_directory else path.read_bytes())
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    record = archive.parent / (archive.name + ".sha256")
    record.write_text(digest + "  " + archive.name + "\n")
    print("wrote %s sha256 %s" % (archive.name, digest))


if __name__ == "__main__":
    main()
