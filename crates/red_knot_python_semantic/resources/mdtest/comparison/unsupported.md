# Comparison: Unsupported operators

```py
a = 1 in 7  # error: "Operator `in` is not supported for types `Literal[1]` and `Literal[7]`"
reveal_type(a)  # revealed: bool

b = 0 not in 10  # error: "Operator `not in` is not supported for types `Literal[0]` and `Literal[10]`"
reveal_type(b)  # revealed: bool

c = object() < 5  # error: "Operator `<` is not supported for types `object` and `int`"
reveal_type(c)  # revealed: Unknown

# TODO should error, need to check if __lt__ signature is valid for right operand
d = 5 < object()
# TODO: should be `Unknown`
reveal_type(d)  # revealed: bool

int_literal_or_str_literal = 1 if flag else "foo"
# error: "Operator `in` is not supported for types `Literal[42]` and `Literal[1]`, in comparing `Literal[42]` with `Literal[1] | Literal["foo"]`"
e = 42 in int_literal_or_str_literal
reveal_type(e)  # revealed: bool

# TODO: should error, need to check if __lt__ signature is valid for right operand
# error may be "Operator `<` is not supported for types `int` and `str`, in comparing `tuple[Literal[1], Literal[2]]` with `tuple[Literal[1], Literal["hello"]]`
f = (1, 2) < (1, "hello")
reveal_type(f)  # revealed: @Todo
```