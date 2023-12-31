# RFC-01: Lumi Programming Language

**Feature Name:** RFC-01: Lumi Programming Language<br/>
**Date:** 2023-12-24<br/>
**Author:** Martin Bucinskas<br/>
**Status:** DRAFT<br/>

## Abstract

Lumi is a general purpose programming language.<br/>
Initially built as a tool of learning and understanding in how VM (virtual-machine)
based languages work and function.

Every language needs to start somewhere.<br/>
This is the starting-point, defining the primitive constructs of the Lumi programming language.

## Table of Contents

- [1. Introduction](#1-introduction)
  - [1.1 Lumi General Purpose Programming Language](#11-lumi-general-purpose-programming-language)
  - [1.2 Lumi Design](#12-lumi-design)
  - [1.3 Definitions](#13-definitions)
- [2. Types](#2-types)
  - [2.1 Primitive Types](#21-primitive-types)
    - [2.1.1 int type](#211-int-type)
    - [2.1.2 float type](#212-float-type)
    - [2.1.3 char type](#213-char-type)
    - [2.1.4 str type](#214-str-type)
    - [2.1.5 bool type](#215-bool-type)
  - [2.2 Type Casting](#22-type-casting)
    - [2.2.1 Syntax](#221-syntax)
- [3. Variables](#3-variables)
  - [3.1 Definition](#31-definition)
  - [3.2 Syntax](#32-syntax)
    - [3.2.1 Variable Names](#321-variable-names)
    - [3.2.2 Initialization](#322-initialization)
    - [3.2.3 Convention](#323-convention)
- [4. General Syntax](#4-general-syntax)
  - [4.1 Line of Code Structure](#41-line-of-code-structure)
  - [4.2 Comments](#42-comments)
    - [4.2.1 Line Comment](#421-line-comment)
    - [4.2.1 Block Comment](#422-block-comment)
    - [4.2.1 Line Doc](#423-line-doc)
  - [4.3 Code Blocks](#43-code-blocks)
  - [4.4 Scope Rules](#44-scope-rules)
- [5. Control Structures](#5-control-structures)
- [6. Functions and Procedures](#6-functions-and-procedures)
- [7. Data Structures and Collections](#7-data-structures-and-collections)
- [8. Operators and Expressions](#8-operators-and-expressions)
- [9. Modules and Namespaces](#9-modules-and-namespaces)
- [10. Standard Library and Built-in Functions](#10-standard-library-and-built-in-functions)
- [11. Concurrency and Parallelism](#11-concurrency-and-parallelism)
- [12. Error Handling and Debugging](#12-error-handling-and-debugging)
- [13. Interoperability](#13-interoperability)
- [14. Code Examples and Best Practices](#14-code-examples-and-best-practices)

## 1. Introduction

### 1.1 Lumi General Purpose Programming Language

There are countless programming languages created,
all serving different purposes, and achieve objectives in their own
ways.

So why the need for yet another programming language?

This is a language created to understand a bit more of what
it actually takes to design and create a programming language.

A lot of inspiration is coming from a blog series designing a VM based
programming language in rust - https://gitlab.com/subnetzero/iridium.

Everything else is based from personal experience and preference.

### 1.2 Lumi Design

The aim for Lumi is to have a high-level programming language,
that is able to run on a VM (virtual-machine) of its own design.
This VM is taking inspiration from works such as the BEAM-VM
which powers Erlang, and the JVM which powers Java.

The concept of write-once, run-anywhere is quite appealing.

Here is some of the design criteria that Lumi is aiming for:
- VM based language
- strongly typed and dynamically typed language
- garbage collected
- follows actor concurrency pattern

### 1.3 Definitions

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
document are to be interpreted as described in [[RFC2119](https://www.rfc-editor.org/rfc/rfc2119)].

## 2. Types

Primitive types are the most basic building blocks of Lumi.

Lumi follows a strongly and dynamically typed approach, where types
_MAY_ change but types _SHALL NOT_ change without being explicitly converted.

### 2.1 Primitive Types

|Type|Name|Description|Example|
|-|-|-|-|
|int|Integer|An integer value without decimal places|`let int_type = 5;`|
|float|Float|A floating point value|`let float_type = 2.0;`|
|char|Character|A single character value|`let char_type = 'a';`|
|str|String|A combination of multiple characters|`let string_type = "hello, world!";`|
|bool|Boolean|A boolean value|`let bool_type = false;`|

#### 2.1.1 int type

An integer value without decimal places.
The integer value **_MUST_** be represented using one of the following constructs:
- base 2: `0b0101001011`
- base 10: `5`
- base 16: `0xF4`

#### 2.1.2 float type

A floating point value.
The floating point value **_MUST_** be represented using one of the following constructs:
- base 10: `2.0`
- base 16: `0x1.921fb54442d18p+0001` - IEEE_754 (https://ieeexplore.ieee.org/document/4610935)

#### 2.1.3 char type

A single character value.
The character value **_MUST_** be defined as a single character surrounded by single quotes, e.g.:
```shell
'a'
```

#### 2.1.4 str type

A combination of multiple characters.
A string value **_MUST_** be defined with double quotes wrapping the actual value, e.g.:
```shell
"hello, world"
```

The actual string value can be empty.

#### 2.1.5 bool type

A boolean value.
The boolean value **_MUST_** be one of the following:
- `false`
- `true`

### 2.2 Type Casting

As mentioned in [[2. Types](#2-types)], Lumi follows a strongly and dynamically typed approach, where types
_MAY_ change but types _SHALL NOT_ change without being explicitly converted.

For example:

```shell
let x = 5;
let message = "Hello";

x = 0;
message = 2; // Throws a compilation failure -> cannot dynamically change type

message = 2 as str; // Casts one type to another
```

#### 2.2.1 Syntax

To cast one type to another, Lumi offers the `as` key word.

**Example casting a direct value to another type:**
```shell
let x = 5 as str;
```

In this example, the integer value of `5` is cast to string, leaving
the variable `x` with the value of `"5"`.

**Example casting a variable value to another type:**
```shell
let x = 5;
let y = x as str;
```

In this example, the variable `x` contains an integer value of `5`.
The value is then cast to a string, leaving the `y` variable value
to equal `"5"`.

## 3. Variables

Every language needs to have a way to define variables.

### 3.1 Definition

A variable can be defined as:
> A variable is an abstract storage location paired with an associated symbolic name,
> which contains some known or unknown quantity of data or object referred to as a value.

### 3.2 Syntax

In Lumi programming language, a variable is defined using the `let` key word.
Every **new** variable **_MUST_** start with the key word `let`, followed with
the variable name.

**Example of variable declaration:**
```shell
let x = "test";
```

#### 3.2.1 Variable Names

Variable names **_MUST_** start with an alphabetic character (`a-Z`) **_OR_** an underscore (`_`).
The rest of the variable name MUST only include the following characters: `a-zA-Z0-9_`.

Giving the following regex pattern for a variable name: `^(([a-zA-Z_]{1})([a-zA-Z0-9]*_*)*)$`.

**Valid variable name examples:**
```shell
foo
foo123
foo_123
_foo_123
foo_
```
**Invalid variable name examples:**
```shell
123
123_foo
%foo
$foo
"foo"
```

#### 3.2.2 Initialization

A variable **_MUST_** be initialized when it is declared.
A variable is initialized with a `=` symbol after the variable name,
followed with the value to initialize the variable with.

**For example:**
```shell
let foo = "bar";
```

In this example, the variable `foo` is initialized with the string value of `"bar"`.

#### 3.2.3 Convention

It is **_RECOMMENDED_** to add an underscore `_` to a variable name to indicate that
that particular variable is not used in any code.

## 4. General Syntax

The general syntax of Lumi takes inspiration from a multitude of languages,
Python, Java, Kotlin, Rust, Go, PHP, just to name a few.

### 4.1 Line of Code Structure

Each line of code **_MUST_** be terminated (ending) with a semicolon `;`.

Example:
```shell
let x = 5;
```

### 4.2 Comments

Every language utilises some form of comments to describe and unravel
the spaghetti code, this language is no different.

#### 4.2.1 Line Comment

Line comments are comments written on a single line.<br/>
A line comment **_MUST_** start with a `//`.

**Example:**
```shell
// this is a line comment
```

#### 4.2.2 Block Comment

A block comment **_MUST_** start with a `/*` and **_MUST_** end with a `*/`.<br />
Block comments **_MAY_** span across multiple lines.<br />
Everything in between of the blocks (`/*` and `*/`) will be considered
as a comment.

**Example:**
```shell
/* This is a block comment on a single line */

/*
This is a block comment spanning
multiple lines
*/
```

#### 4.2.3 Line Doc

A line doc is a comment that is used as part of documentation.<br />
A line doc **_MUST_** be declared with a `///`. Line docs **_SHOULD_** be placed before
the line of code it is referring to.

**Example:**
```shell
/// The value of PI.
let pi = 3.14;
```

Multiple line docs **_MAY_** be combined to create a single doc.

**Example:**
```shell
/// The value of PI.
/// This value is an approximation.
let pi = 3.14;
```

### 4.3 Code Blocks

Code blocks are used for control logic, function definitions, etc.

Code blocks are defined using parentheses `{}`.<br />
A code block **_MUST_** start with a `{` and **_MUST_** end with a matching `}`.

A code block **_MAY_** span across multiple lines, or it **_MAY_** span across
the same line as the opening brace.

**Example:**
```shell
if x > 5 {
  // do something
};

if x > 5 { /* do something */ };
```

### 4.4 Scope Rules

Variables and functions declared throughout the code are scoped.
In computer programming, the scope of a name binding is the part of
a program where the name binding is valid; that is, where the name
can be used to refer to the entity. In other parts of the program,
the name may refer to a different entity, or to nothing at all.

A variable or a function, or any other scoped construct **_MAY_** be referred
to in the current, or any subsequent block of code.

A variable or a function **_SHALL NOT_** be redeclared in the current
block level or any subsequent block levels if it is already
declared in the current scope.

**Example:**
```shell
let x = 5;

if x >= 5 {
  x = 2; // x is already declared one block level above, so this works
  let x = 2; // x is already declared, so this fails to be redeclared
};
```

## 5. Control Structures
- Conditional Statements (if, else, switch-case)
- Loops (for, while, do-while)
- Exception Handling (try, catch, finally)

## 6. Functions and Procedures
- Function Declaration and Definition
- Parameters and Return Types
- Function Overloading
- Anonymous Functions/Lambdas

## 7. Data Structures and Collections
- Arrays and Lists
- Maps/Dictionaries
- Sets
- Custom Data Structures (structs, classes, etc)

## 8. Operators and Expressions
- Arithmetic Operators
- Logical Operators
- Comparison Operators
- Assignment Operators
- Precedence and Associativity

## 9. Modules and Namespaces
- Module/Package Definition
- Importing and Exporting Modules
- Namespace Management

## 10. Standard Library and Built-in Functions
- Overview of Built-in Functions
- Common Libraries (e.g., Math, String processing, I/O operations)

## 11. Concurrency and Parallelism
- Threads, Goroutines, Async/Await
- Messaging between actors
- Synchronization mechanisms (mutex, lock)

## 12. Error Handling and Debugging
- Error Handling Mechanisms
- Debugging Tools and Techniques

## 13. Interoperability
- Interaction with other languages or systems
- Foreign Function Interface (FFI)

## 14. Code Examples and Best Practices
- Sample programs
- Coding Guidelines and Style Recommendations
