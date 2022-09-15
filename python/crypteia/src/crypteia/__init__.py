import wrapt

from ctypes import CDLL, c_char_p
getenv = CDLL("libc.so.6").getenv
getenv.restype = c_char_p
getenv("HOME")

# USE WRAPT TO REIMPLEMENT os.environ

# Use "/tmp/crypteia.json" file?
