# Trigger the load of the Crypteia python package,
# that is going to apply wrapt to the os.environ object,
# so that we can customize the lookup of the environment
# variables at runtime. We cannot do this in LD_PRELOAD,
# because CPython uses LibC to look up only the PYTHON*
# environment variables, parsing directly the memory
# resolved by the linker to the __environ symbol as a
# Unix dictionary.

# IMPORTANT: This file must be valid Python 2.7+

from os.path import dirname
from sys import path, version, version_info


def import_crypteia():
    current_site = dirname(__file__)

    # We cannot use `sys.version_info.major` and other named attribute
    # got introduced only in Python 3.1
    if version_info[0] == 2 and version_info[1] < 7:
        path.remove(current_site)
        print("#CRYPTEIA# - Cannot import 'crypteia' due unsupported runtime version: {}".format(version))
        return

    import crypteia


import_crypteia()
