# Lisp interpreter

Attempts of writing a Lisp interpreter.

## Examples

Writing to standard output with `write`.

```shell
$ lisp "(write 1)"
1
```

Quoting expressions with `quote`.

```shell
$ lisp "(write (quote (hello world)))"
(hello world)
```

Adding numbers with `add`.

```shell
$ lisp "(write (add 1 2 3 4))"
10
```

Defining variables with `let`.

```shell
$ lisp "(let x 2) (let y (add x x x)) (write y)"
6
```

Evaluating defined variable with `eval`.

```shell
$ lisp "(let x (quote (add 1 2 3))) (write x) (write (eval x))"
(add 1 2 3)
6
```
