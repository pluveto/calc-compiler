# Design

下面的代码：

```c
long long mem = 0;
int printf ( const char * format, ... );

int main() {
    mem++;
    printf("%lld\n", mem);
    int a = 42 * mem;
    return a;
}
```

对应于下面的 LLVM IR

```llvm
@mem = global i64 0
@.str = private unnamed_addr constant [6 x i8] c"%lld\0A\00"

define i32 @main() {
  %1 = alloca i32
  %2 = alloca i32
  store i32 0, %1
  %3 = load i64, @mem
  %4 = add nsw i64 %3, 1
  store i64 %4, @mem
  %5 = load i64, @mem
  %6 = call i32 (ptr, ...) @printf(ptr @.str, i64 %5)
  %7 = load i64, @mem
  %8 = mul nsw i64 42, %7
  %9 = trunc i64 %8 to i32
  store i32 %9, %2
  %10 = load i32, %2
  ret i32 %10
}

declare i32 @printf(ptr, ...)
```

这些代码由一系列基本块（basic block）、指令（instruction）、函数（function）、全局变量（global variable）等构成。这些构成成分在 LLVM 中通常被称为“IR Entity”

在 LLVM IR 中，所有的实体（entity）都被表示为 Value 类型。Value 是 LLVM IR 中的基本数据类型，表示任何可以被命名、使用、传递和操作的实体，包括常量、指令、函数参数、全局变量等等。

# Value 和 ValueId

## Value

Value 意味着一个带有编号的，对具体实体的引用。

例如我们有一个 LoadInst: `load i32, i32* %0`，这尚不能构成一个 value，因为它没有编号，我们需要给它分配一个编号，例如 `%1 = load i32, i32* %0`，这样就构成了一个 value。

`%1` 称为 这个 InstValue 的名称。

上面指令中还有 `%0`，这是此 LoadInst 所依赖的一个 Value 的名称。这个名称对应的 Value 可能是一个全局变量，也可能是一个函数参数，也可能是一个指令的结果。

例子：

- 在 `load i32, ptr @mem, align 4, !dbg !21` 中，`@mem` 是一个全局变量。
- 在 `load i32, ptr %2, align 4, !dbg !20` 中，`%2` 是一个函数参数或局部变量。

## ValueId

ValueId 并不是一个 Value，也不是一个 Value 的编号。

下面的例子