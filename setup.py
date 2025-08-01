""" based on
    https://github.com/pybind/python_example/blob/master/setup.py
"""
from setuptools import setup, errors, Extension
from setuptools.command.build_ext import build_ext
import sys


class get_pybind_include(object):
    """Helper class to determine the pybind11 include path

    The purpose of this class is to postpone importing pybind11
    until it is actually installed, so that the ``get_include()``
    method can be invoked."""

    def __init__(self, user=False):
        self.user = user

    def __str__(self):
        import pybind11

        return pybind11.get_include(self.user)


ext_modules = [
    Extension(
        "chiabip158",
        [
            "src/blockfilter.cpp",
            "src/crypto/sha256.cpp",
            "src/crypto/siphash.cpp",
            "src/primitives/block.cpp",
            "src/primitives/transaction.cpp",
            "src/script/script.cpp",
            "src/util/strencodings.cpp",
            "src/util/bytevectorhash.cpp",
            "src/uint256.cpp",
            "python-bindings/PyBIP158.cpp",
            "python-bindings/chiabip158.cpp",
        ],
        include_dirs=[
            # Path to pybind11 headers
            get_pybind_include(),
            get_pybind_include(user=True),
            "src",
        ],
        language="c++",
    ),
]


# As of Python 3.6, CCompiler has a `has_flag` method.
# cf http://bugs.python.org/issue26689
def has_flag(compiler, flagname):
    """Return a boolean indicating whether a flag name is supported on
    the specified compiler.
    """
    import tempfile

    with tempfile.NamedTemporaryFile("w", suffix=".cpp") as f:
        f.write("int main (int argc, char **argv) { return 0; }")
        try:
            compiler.compile([f.name], extra_postargs=[flagname])
        except errors.CompileError:
            return False
    return True


def cpp_flag(compiler):
    """Return the -std=c++[11/14/17] compiler flag.

    The newer version is prefered over c++11 (when it is available).
    """
    flags = ["-std=c++17", "-std=c++14", "-std=c++11"]

    for flag in flags:
        if has_flag(compiler, flag):
            return flag

    raise RuntimeError("Old compiler -- at least C++11 support is needed!")


class BuildExt(build_ext):
    """A custom build extension for adding compiler-specific options."""

    c_opts = {
        "msvc": ["/EHsc"],
        "unix": [],
    }
    l_opts = {  # type: ignore
        "msvc": [],
        "unix": [],
    }

    if sys.platform == "darwin":
        darwin_opts = ["-stdlib=libc++"]
        c_opts["unix"] += darwin_opts  # type: ignore
        l_opts["unix"] += darwin_opts  # type: ignore

    def build_extensions(self):
        ct = self.compiler.compiler_type
        opts = self.c_opts.get(ct, [])
        link_opts = self.l_opts.get(ct, [])
        if ct == "unix":
            opts.append('-DVERSION_INFO="%s"'
                        % self.distribution.get_version())
            opts.append(cpp_flag(self.compiler))
            if has_flag(self.compiler, "-fvisibility=hidden"):
                opts.append("-fvisibility=hidden")
        elif ct == "msvc":
            opts.append('/DVERSION_INFO=\\"%s\\"'
                        % self.distribution.get_version())
        for ext in self.extensions:
            ext.extra_compile_args = opts
            ext.extra_link_args = link_opts
        build_ext.build_extensions(self)


setup(
    name="chiabip158",
    author="Mariano Sorgente",
    author_email="mariano@chia.net",
    description="Chia BIP158 (wraps C++)",
    license="Apache License",
    python_requires=">=3.7",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    setup_requires=["pybind11>=2.10.0"],
    url="https://github.com/Chia-Network/chiabip158",
    ext_modules=ext_modules,
    cmdclass={"build_ext": BuildExt},
    zip_safe=False,
)
