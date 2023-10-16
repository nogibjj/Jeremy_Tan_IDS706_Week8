"""argument handler """
from base64 import b64encode, decodebytes
import argparse
import time
from mylib.lib import encrypt_aes_256_cbc_pkcs, decrypt_aes_256_cbc_pkcs, log_encrypt


def main():
    """handles arguments"""

    start_time = time.perf_counter()
    parser = argparse.ArgumentParser(
        description="Encrypt and decrypt text using AES-256 CBC PKCS#7"
    )

    parser.add_argument(
        "mode", choices=["encrypt", "decrypt"], help="Mode: encrypt or decrypt"
    )
    parser.add_argument("input_text", type=str, help="Input text")
    parser.add_argument(
        "--key",
        type=str,
        default=b"C/C7ZVYfatWe6DTz9ZX6gfmYrQFSqfLuHFhDun9YgfU=",
        help="Key in base64 format (32 bytes for AES-256)",
    )
    parser.add_argument(
        "--iv",
        type=str,
        default=b"q/ohmAU6as5SDVuQRkPLWw==",
        help="Initialization Vector (IV) in base64 format (16 bytes)",
    )

    args = parser.parse_args()

    if args.mode == "encrypt":
        ciphertext = encrypt_aes_256_cbc_pkcs(
            (args.input_text).encode("utf-8"),
            decodebytes(args.key),
            decodebytes(args.iv),
        )
        print("Encrypted text:", b64encode(ciphertext).decode())
        end_time = time.perf_counter()
        elapsed_time_micros = (end_time - start_time) * 1e6
        print(f"Elapsed time: {elapsed_time_micros} microseconds")

        log_encrypt(
            args.input_text, True, b64encode(ciphertext).decode(), elapsed_time_micros
        )

    elif args.mode == "decrypt":
        print((args.input_text).encode("utf-8"))
        decrypted_data = decrypt_aes_256_cbc_pkcs(
            decodebytes((args.input_text).encode("utf-8")),
            decodebytes(args.key),
            decodebytes(args.iv),
        )
        print("Decrypted text:", decrypted_data.decode())
        end_time = time.perf_counter()
        elapsed_time_micros = (end_time - start_time) * 1e6
        print(f"Elapsed time: {elapsed_time_micros} microseconds")

        log_encrypt(
            args.input_text, False, decrypted_data.decode(), elapsed_time_micros
        )


if __name__ == "__main__":
    main()
