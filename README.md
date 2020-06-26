# Chia BIP158
![Build](https://github.com/Chia-Network/chiabip158/workflows/Build/badge.svg)
![PyPI](https://img.shields.io/pypi/v/chiabip158?logo=pypi)
![PyPI - Format](https://img.shields.io/pypi/format/chiabip158?logo=pypi)
![GitHub](https://img.shields.io/github/license/Chia-Network/chiabip158?logo=Github)

[![Total alerts](https://img.shields.io/lgtm/alerts/g/Chia-Network/chiabip158.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/Chia-Network/chiabip158/alerts/)
[![Language grade: Python](https://img.shields.io/lgtm/grade/python/g/Chia-Network/chiabip158.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/Chia-Network/chiabip158/context:python)
[![Language grade: C/C++](https://img.shields.io/lgtm/grade/cpp/g/Chia-Network/chiabip158.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/Chia-Network/chiabip158/context:cpp)

This implements the compact block filter construction in BIP 158. The code is
not used anywhere in the Bitcoin Core code base yet. The next step towards
BIP 157 support would be to create an indexing module similar to TxIndex that
constructs the basic and extended filters for each validated block.

## Install

```bash
python3 -m venv venv
. venv/bin/activate
pip3 install .
```

## Run python tests

```bash
python3 tests/simple_test.py
```

## Installation steps on a fresh OSX image

Install brew:

```bash
ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

brew install python3  
brew install boost  
```

At this point the only error is can’t find boost_thread lib

The issue is the homebrew boost ships libboost_thread-mt libs but doesn’t
include plain libboost_thread, so clang can’t find it. Interestingly, homebrew
boost does have both plain and -mt files for the libboost_system libraries.

```bash
$ find /usr/local/lib/ | grep boost_thread  
libboost_thread-mt.a  
libboost_thread-mt.dylib  
```

Solution, with no guarantees that this is "the Right Way to do things", but
appears to work fine for the configure stage:

```bash
cd /usr/local/lib  
ln -s libboost_thread-mt.a libboost_thread.a  
ln -s libboost_thread-mt.dylib libboost_thread.dylib  
```
