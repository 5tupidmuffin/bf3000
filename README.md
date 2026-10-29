### bf3000 - a brainfuck transpiler
#### usage
```shell
bf3000 [bf_script]
# or, to get compiled binary
CC=gcc|clang CFLAGS=-O2 bf3000 [bf_script]
```
if a C Compiler path is set via CC path variable, bf3000 will try to compile the generated c code with it.

#### references
- optimization referenced from http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html.
