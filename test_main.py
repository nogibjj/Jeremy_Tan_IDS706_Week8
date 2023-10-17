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

    result = subprocess.run(
        [
            "python",
            "main.py",
            "decrypt",
            "Q9LCh9YvaLPGcXeEMQ8bSWz2nU0FMoqYmqBl0GxX6D1MXj76zHXboHyrDMZHeiR0H2MiDekXUV1FBeZYBUivc++YYzD78RhBMfpP6lq+IttySE0ns/P/xueKX4wY3ln8sBy/b1Zrm52Lun5KqmFhZvHj7d2pwHsz7DiuK37TkHg=",
        ],
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

    result = subprocess.run(
        ["python", "main.py", "encrypt", "Mary had a little lamb a litte lamb"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":
    test_encrypt()
    test_decrypt()
