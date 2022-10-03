import ctypes
import os
import sys

from crypteia.wrapt import ObjectProxy, register_post_import_hook, wrap_object

PY2 = sys.version_info[0] == 2

crypteia = os.environ.get("LD_PRELOAD")
getenv = ctypes.cdll.LoadLibrary(crypteia).getenv
getenv.restype = ctypes.c_char_p


class GetEnvWrapper(ObjectProxy):
    def __getitem__(self, key):
        if PY2:
            value = getenv(key)
        else:
            encodedkey = self.__wrapped__.encodekey(key)
            value = getenv(encodedkey)
            if value:
                value = self.__wrapped__.decodevalue(value)

        if value:
            return value
        else:
            raise KeyError(key)

    def __repr__(self):
        if PY2:
            return repr(self.__wrapped__.data)
        else:
            return repr(self.__wrapped__)

    def get(self, key, default=None):
        try:
            item = self[key]
        except KeyError:
            return default

        return item


def _load_and_patch(module):
    wrap_object(module, 'environ', GetEnvWrapper)


register_post_import_hook(_load_and_patch, 'os')
