"""Python 'utf-16-le' Codec


Written by Marc-Andre Lemburg (mal@lemburg.com).

(c) Copyright CNRI, All Rights Reserved. NO WARRANTY.

"""

import codecs
from _typeshed import ReadableBuffer

encode = codecs.utf_16_le_encode

def decode(input: ReadableBuffer, errors: str | None = "strict") -> tuple[str, int]: ...

class IncrementalEncoder(codecs.IncrementalEncoder):
    def encode(self, input: str, final: bool = False) -> bytes: ...

class IncrementalDecoder(codecs.BufferedIncrementalDecoder):
    # At runtime, this is codecs.utf_16_le_decode
    @staticmethod
    def _buffer_decode(data: ReadableBuffer, errors: str | None = None, final: bool = False, /) -> tuple[str, int]: ...

class StreamWriter(codecs.StreamWriter):
    # At runtime, this is codecs.utf_16_le_encode
    @staticmethod
    def encode(str: str, errors: str | None = None, /) -> tuple[bytes, int]: ...

class StreamReader(codecs.StreamReader):
    # At runtime, this is codecs.utf_16_le_decode
    @staticmethod
    def decode(data: ReadableBuffer, errors: str | None = None, final: bool = False, /) -> tuple[str, int]: ...

def getregentry() -> codecs.CodecInfo: ...
