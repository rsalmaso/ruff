import socket
import sys
from _typeshed import ReadableBuffer
from builtins import type as Type  # alias to avoid name clashes with property named "type"
from collections.abc import Iterable
from types import TracebackType
from typing import Any, BinaryIO, NoReturn, overload
from typing_extensions import TypeAlias

# These are based in socket, maybe move them out into _typeshed.pyi or such
_Address: TypeAlias = socket._Address
_RetAddress: TypeAlias = Any
_WriteBuffer: TypeAlias = bytearray | memoryview
_CMSG: TypeAlias = tuple[int, int, bytes]

class TransportSocket:
    """A socket-like wrapper for exposing real transport sockets.

    These objects can be safely returned by APIs like
    `transport.get_extra_info('socket')`.  All potentially disruptive
    operations (like "socket.close()") are banned.
    """

    def __init__(self, sock: socket.socket) -> None: ...
    @property
    def family(self) -> int: ...
    @property
    def type(self) -> int: ...
    @property
    def proto(self) -> int: ...
    def __getstate__(self) -> NoReturn: ...
    def fileno(self) -> int: ...
    def dup(self) -> socket.socket: ...
    def get_inheritable(self) -> bool: ...
    def shutdown(self, how: int) -> None: ...
    @overload
    def getsockopt(self, level: int, optname: int) -> int: ...
    @overload
    def getsockopt(self, level: int, optname: int, buflen: int) -> bytes: ...
    @overload
    def setsockopt(self, level: int, optname: int, value: int | ReadableBuffer) -> None: ...
    @overload
    def setsockopt(self, level: int, optname: int, value: None, optlen: int) -> None: ...
    def getpeername(self) -> _RetAddress: ...
    def getsockname(self) -> _RetAddress: ...
    def getsockbyname(self) -> NoReturn: ...  # This method doesn't exist on socket, yet is passed through?
    def settimeout(self, value: float | None) -> None: ...
    def gettimeout(self) -> float | None: ...
    def setblocking(self, flag: bool) -> None: ...
    if sys.version_info < (3, 11):
        def _na(self, what: str) -> None: ...
        def accept(self) -> tuple[socket.socket, _RetAddress]: ...
        def connect(self, address: _Address) -> None: ...
        def connect_ex(self, address: _Address) -> int: ...
        def bind(self, address: _Address) -> None: ...
        if sys.platform == "win32":
            def ioctl(self, control: int, option: int | tuple[int, int, int] | bool) -> None: ...
        else:
            def ioctl(self, control: int, option: int | tuple[int, int, int] | bool) -> NoReturn: ...

        def listen(self, backlog: int = ..., /) -> None: ...
        def makefile(self) -> BinaryIO: ...
        def sendfile(self, file: BinaryIO, offset: int = ..., count: int | None = ...) -> int: ...
        def close(self) -> None: ...
        def detach(self) -> int: ...
        if sys.platform == "linux":
            def sendmsg_afalg(
                self, msg: Iterable[ReadableBuffer] = ..., *, op: int, iv: Any = ..., assoclen: int = ..., flags: int = ...
            ) -> int: ...
        else:
            def sendmsg_afalg(
                self, msg: Iterable[ReadableBuffer] = ..., *, op: int, iv: Any = ..., assoclen: int = ..., flags: int = ...
            ) -> NoReturn: ...

        def sendmsg(
            self, buffers: Iterable[ReadableBuffer], ancdata: Iterable[_CMSG] = ..., flags: int = ..., address: _Address = ..., /
        ) -> int: ...
        @overload
        def sendto(self, data: ReadableBuffer, address: _Address) -> int: ...
        @overload
        def sendto(self, data: ReadableBuffer, flags: int, address: _Address) -> int: ...
        def send(self, data: ReadableBuffer, flags: int = ...) -> int: ...
        def sendall(self, data: ReadableBuffer, flags: int = ...) -> None: ...
        def set_inheritable(self, inheritable: bool) -> None: ...
        if sys.platform == "win32":
            def share(self, process_id: int) -> bytes: ...
        else:
            def share(self, process_id: int) -> NoReturn: ...

        def recv_into(self, buffer: _WriteBuffer, nbytes: int = ..., flags: int = ...) -> int: ...
        def recvfrom_into(self, buffer: _WriteBuffer, nbytes: int = ..., flags: int = ...) -> tuple[int, _RetAddress]: ...
        def recvmsg_into(
            self, buffers: Iterable[_WriteBuffer], ancbufsize: int = ..., flags: int = ..., /
        ) -> tuple[int, list[_CMSG], int, Any]: ...
        def recvmsg(self, bufsize: int, ancbufsize: int = ..., flags: int = ..., /) -> tuple[bytes, list[_CMSG], int, Any]: ...
        def recvfrom(self, bufsize: int, flags: int = ...) -> tuple[bytes, _RetAddress]: ...
        def recv(self, bufsize: int, flags: int = ...) -> bytes: ...
        def __enter__(self) -> socket.socket: ...
        def __exit__(
            self, exc_type: Type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None
        ) -> None: ...
