from chiabip158 import PyBIP158
from hashlib import sha256
import random

random.seed(158)


def test_rust_equivalency():
    elem1 = bytearray(sha256(b"abc").digest())
    elem2 = bytearray(sha256(b"xyz").digest())
    elem3 = bytearray(sha256(b"123").digest())
    not_elem1 = bytearray(sha256(b"hello").digest())
    not_elem2 = bytearray(sha256(b"bye").digest())

    filter = PyBIP158([elem1, elem2, elem3])

    encoded = filter.GetEncoded()
    assert encoded == [3, 174, 90, 204, 224, 219, 7, 253, 91]

    assert filter.Match(elem1)
    assert filter.Match(elem2)
    assert filter.Match(elem3)

    assert not filter.Match(not_elem1)
    assert not filter.Match(not_elem2)

    assert filter.MatchAny([elem1, elem2, elem3])
    assert filter.MatchAny([elem1, elem2])
    assert filter.MatchAny([elem3])
    assert filter.MatchAny(
        [not_elem1, not_elem1, elem1, not_elem2, not_elem1, not_elem2, elem2]
    )

    assert not filter.MatchAny([])
    assert not filter.MatchAny([not_elem1, not_elem2])


def test_simple():
    print("BIP 158 test")

    hsharray = []

    for i in range(1000):
        hsh = bytearray(sha256(i.to_bytes(4, "big")).digest())
        hsharray.append(hsh)

    pl = PyBIP158(hsharray)

    while True:
        print("*** Match Test ***")
        matcharray = []
        for j in range(10):
            rando = random.randint(0, 6000)
            matchhash = bytearray(sha256(rando.to_bytes(4, "big")).digest())
            if pl.Match(matchhash):
                print(str(rando) + " OK")
        else:
            print(str(rando) + " not found")
            matcharray.append(matchhash)
        if pl.MatchAny(matcharray):
            print("OK")
        else:
            print("NONE FOUND")
            break
