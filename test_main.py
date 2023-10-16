"""
Test goes here

"""

import subprocess


def test_decrypt():
    """tests decrypt"""
    result = subprocess.run(
        ["python", "main.py", "decrypt", "Gb/Ve9PhJ665clfO5DMi3g=="],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_encrypt():
    """tests encrypt"""
    result = subprocess.run(
        ["python", "main.py", "encrypt", "Hello World"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":
    test_encrypt()
    test_decrypt()
