"""libaray functions for encrpytion and decryption"""
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import padding

# Define a global variable for the log file
LOG_FILE = "python_times.md"


def log_encrypt(message, encrypt, result, time, memory_used):
    """adds to a query markdown file"""
    cipher = "encrypt" if encrypt else "decrypt"

    with open(LOG_FILE, "a") as file:
        file.write(f"\nThe orginal message to the cipher is {message}\n\n\n")
        file.write(f"The result of the {cipher} is {result}\n\n\n")
        file.write(f"Elapsed time: {time} microseconds\n\n\n")
        file.write(f"- Memory used: {memory_used} kB\n")


def encrypt_aes_256_cbc_pkcs(data, key, iv):
    """encrypt data with key and iv"""
    padder = padding.PKCS7(128).padder()
    padded_data = padder.update(data) + padder.finalize()

    backend = default_backend()
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=backend)
    encryptor = cipher.encryptor()
    ciphertext = encryptor.update(padded_data) + encryptor.finalize()

    return ciphertext


def decrypt_aes_256_cbc_pkcs(ciphertext, key, iv):
    """decrypt data with key and iv"""
    backend = default_backend()
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=backend)
    decryptor = cipher.decryptor()
    padded_data = decryptor.update(ciphertext) + decryptor.finalize()

    unpadder = padding.PKCS7(128).unpadder()
    data = unpadder.update(padded_data) + unpadder.finalize()

    return data
