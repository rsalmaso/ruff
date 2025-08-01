"""
Python 'utf-32' Codec
"""

import codecs
from _typeshed import ReadableBuffer

encode = codecs.utf_32_encode

def decode(input: ReadableBuffer, errors: str | None = "strict") -> tuple[str, int]: ...

class IncrementalEncoder(codecs.IncrementalEncoder):
    def encode(self, input: str, final: bool = False) -> bytes: ...

class IncrementalDecoder(codecs.BufferedIncrementalDecoder):
    def _buffer_decode(self, input: ReadableBuffer, errors: str, final: bool) -> tuple[str, int]: ...

class StreamWriter(codecs.StreamWriter):
    def encode(self, input: str, errors: str = "strict") -> tuple[bytes, int]: ...

class StreamReader(codecs.StreamReader):
    def decode(self, input: ReadableBuffer, errors: str = "strict") -> tuple[str, int]: ...

def getregentry() -> codecs.CodecInfo: ...
