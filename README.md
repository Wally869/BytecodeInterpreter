# Interpreter  

Bytecode interpreter to meet the following requirements:   

```
(1) You are a TA at a university, and you want to evaluate your student’s homework
without executing their (untrusted) code. You decide to write a small
web-service that takes bytecode as input, and interprets the results.
The bytecode language you need to support includes basic arithmetic and
variables. The bytecode language is stack, rather than register based.
ByteCode (right) is given for the following pseudo code (left):  

function f() {
x = 1 LOAD_VAL 1
WRITE_VAR ‘x’
y = 2 LOAD_VAL 2
WRITE_VAR ‘y’
return (x + 1) * y READ_VAR ‘x’
LOAD_VAL 1
ADD
READ_VAR ‘y’
MULTIPLY
RETURN_VALUE
}  


Add a data type `ByteCode` that can represent bytecode like in the example
above, along with an interpreter for said bytecode. Make sure your bytecode
is flat, i.e. not nested.
(2) Extend your interpreter with loops. In particular:
(a) Extend your `ByteCode` data type with suitable instructions to support loops
(b) Modify your interpreter to support said instructions
(c) Try it out and see if it works :)

```

## About
The interpreter uses an instruction pointer and jump instructions to handle loops (jump, jump if).  
Tests cover basic operation as well as a increment loop.  

Named variables are stored as variables on the stack but since walking the stack is expensive, could also store them in a hashmap since stack machines usually have RAM.    
