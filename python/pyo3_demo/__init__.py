from .pyo3_demo import *

__doc__ = pyo3_demo.__doc__
if hasattr(pyo3_demo, "__all__"):
    __all__ = pyo3_demo.__all__

# It is a common practice in Python packaging to keep the extension modules
# private and use Pure Python modules to wrap them.
# This allows you to have a very fine control over the public API.
