# Lumi Language

Based on actor concurrency pattern

## Types
```shell
let int_type = 5; // int
let float_type = 2.0; // float
let string_type = "hello world"; // string
let char_type = 'c'; // single character
let bool_type = false; // boolean (true, false)
```

## Variables
```shell
let x = 5;
let message = "Hello";

x = 0;
message = 2; // Throws a compilation failure -> cannot dynamically change type

message = 2 as str; // Casts one type to another
```

## Arithmetic
```shell
let x = 5;
let y = -2;
let z = x + y; // add
let z = x - y; // sub
let z = x * y; // mul
let z = x / y; // div
let z = x % y; // mod
let z = x ^^ y; // exponent
```

## Bitwise
```shell
let x = 5;
let y = 2;
let z = ~x; // not
let z = x & y; // and
let z = x | y; // or
let z = x ^ y; // xor
let z = x >> y; // shift right
let z = x << y; // shift left
```

## Comparison
```shell
let x = 5;
let y = 2;
let z = x > y;
let z = x >= y;
let z = x < y;
let z = x <= y;
let z = x == y;
```

## Combinations
```shell
let x = 5;
let y = 2;
let result = (x + 2) / y;

let check = (x == 5) && (y > 2);
let check = (x > 5) || (y <= 2); 
```