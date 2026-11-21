"""Builds a CRX3 package (the format Chrome installs extensions from), and derives the
extension ID from the signing key. No dependencies beyond `cryptography`.

CRX3 layout (chromium/src/components/crx_file/crx3.proto):
  "Cr24" | uint32le(3) | uint32le(header length) | CrxFileHeader | zip
  CrxFileHeader         : field 2 repeated AsymmetricKeyProof sha256_with_rsa,
                          field 10000 bytes signed_header_data
  AsymmetricKeyProof    : field 1 bytes public_key, field 2 bytes signature
  CrxSignedHeaderData   : field 1 bytes crx_id (16)
  signature = RSA-SHA256 over "CRX3 SignedData\\0" + uint32le(len(signed_header_data))
              + signed_header_data + zip
"""
from __future__ import annotations

import base64
import hashlib
import io
import pathlib
import struct
import sys
import zipfile

from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, rsa

MAGIC = b"Cr24"
SIGNED_DATA_PREFIX = b"CRX3 SignedData\x00"


def _varint(value: int) -> bytes:
    out = bytearray()
    while True:
        b = value & 0x7F
        value >>= 7
        out.append(b | (0x80 if value else 0))
        if not value:
            return bytes(out)


def _field(number: int, payload: bytes) -> bytes:
    return _varint((number << 3) | 2) + _varint(len(payload)) + payload


def extension_id(public_key_der: bytes) -> str:
    """Chrome's ID: sha256 of the SPKI DER, first 16 bytes, each hex digit shifted a-p."""
    digest = hashlib.sha256(public_key_der).hexdigest()[:32]
    return "".join(chr(ord("a") + int(c, 16)) for c in digest)


def public_key_der(key: rsa.RSAPrivateKey) -> bytes:
    return key.public_key().public_bytes(
        encoding=serialization.Encoding.DER,
        format=serialization.PublicFormat.SubjectPublicKeyInfo,
    )


def load_or_make_key(path: pathlib.Path) -> rsa.RSAPrivateKey:
    if path.is_file():
        return serialization.load_pem_private_key(path.read_bytes(), password=None)
    key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(
        key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
        )
    )
    return key


def zip_folder(folder: pathlib.Path) -> bytes:
    """The extension folder as a zip, manifest.json at the root, '/' separators."""
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as archive:
        for file in sorted(folder.rglob("*")):
            if file.is_file() and not file.name.startswith("."):
                archive.write(file, file.relative_to(folder).as_posix())
    return buffer.getvalue()


def build_crx(folder: pathlib.Path, key: rsa.RSAPrivateKey) -> bytes:
    der = public_key_der(key)
    crx_id = bytes.fromhex("".join(format(ord(c) - ord("a"), "x") for c in extension_id(der)))
    signed_header = _field(1, crx_id)
    payload = zip_folder(folder)
    signed = SIGNED_DATA_PREFIX + struct.pack("<I", len(signed_header)) + signed_header + payload
    signature = key.sign(signed, padding.PKCS1v15(), hashes.SHA256())
    proof = _field(1, der) + _field(2, signature)
    header = _field(2, proof) + _field(10000, signed_header)
    return MAGIC + struct.pack("<I", 3) + struct.pack("<I", len(header)) + header + payload


def main() -> int:
    args = sys.argv[1:]
    if len(args) < 3:
        print("usage: crx3.py <extension-folder> <out.crx> <private-key.pem> [--print-id]")
        return 2
    folder = pathlib.Path(args[0])
    out = pathlib.Path(args[1])
    key = load_or_make_key(pathlib.Path(args[2]))
    if not (folder / "manifest.json").is_file():
        print(f"error: {folder} has no manifest.json")
        return 1
    data = build_crx(folder, key)
    out.write_bytes(data)
    der = public_key_der(key)
    print(f"id:    {extension_id(der)}")
    print(f"key:   {base64.b64encode(der).decode()}")
    print(f"crx:   {out} ({len(data)} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
