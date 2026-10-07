# Idiomatic Rust rules for the Kaleidoscope port

Goal: implement the LLVM Kaleidoscope tutorial as idiomatic Rust, not a line-by-line port of the C++.

## Core rule
Every C++ global (`CurTok`, `LastChar`, `BinopPrecedence`, `NamedValues`, `TheModule`, `Builder`, `FunctionProtos`, `TheJIT`) becomes a field on whatever owns it: `Lexer`, `Parser`, `CodeGen<'ctx>` or a `Session`. Reaching for `static mut`, `thread_local!` or `Rc<RefCell<…>>` means C++'s pointer layout is being copied; change who owns what instead.

## Structure
- `lib.rs` with modules (`lexer`, `ast`, `parser`, `codegen`, `jit`); `main.rs` stays small. Lexer, parser and AST have no LLVM dependency, so they build and test fast.
- The AST is plain data. Codegen is a separate pass (`CodeGen::expr(&mut self, &Expr)` built on a `match`), not a `codegen()` method or trait on each node.
- No over-abstraction: no visitor trait, no `trait Expr`, no generics with a single implementation.

## Lexer
- Lexes a `&str` through `Peekable<CharIndices>`; the driver does I/O. Gives correct UTF-8, byte offsets for error locations, and easy tests.
- `impl Iterator` yielding `Result<…, LexError>`; end of input is `None` (no `Token::Eof`).
- Bad input returns an `Err`, never a default value.
- Source locations are tracked from the start (C++ adds `SourceLocation` in ch. 9 through another global).

## AST
- Enums with exhaustive `match`; no `_ =>` arms on our own enums, so new variants in ch. 5–7 make the compiler list every site to update.
- Invalid states are unrepresentable: `enum ProtoKind { Function, Unary(char), Binary { op: char, prec: u8 } }` instead of C++'s `bool IsOperator; unsigned Precedence;`. Optional parts (`for` step, `var` initializer) are `Option`.
- Top level: `enum Item { Def(Function), Extern(Prototype), Expr(Expr) }`. Wrapping a bare expression in `__anon_expr` is the driver's job, not the parser's.

## Parser
- Holds a `Peekable` token stream; `peek`, `next_if` and an `expect(tok)?` helper replace `CurTok` + `getNextToken()`.
- Every parse function returns `Result<_, ParseError>` and uses `?`. The parser never prints; C++'s "log an error, return `nullptr`" becomes an error value the driver reports.
- Typed error enums (`Display` or `thiserror`), not `Result<T, String>`. `ParseError::UnexpectedEof` also tells the REPL to read another line.
- Precedence is syntax: the parser owns or `&mut`-borrows the table and registers `def binary…` precedences itself. Lookups return `Option<u8>`, not `-1`. Codegen never touches the table (C++ writes it from codegen).

## Codegen
- `inkwell`, not raw `llvm-sys`. `unsafe` only at the JIT call.
- `CodeGen<'ctx>` holds `&'ctx Context`, `Module<'ctx>`, `Builder<'ctx>`. Don't escape `'ctx` with `'static` or `Box::leak`.
- `impl From<BuilderError>` for the codegen error type; `?` on every `build_*` call, no `unwrap()`.
- Fallible loops collect: `args.iter().map(|a| self.expr(a)).collect::<Result<Vec<_>, _>>()?` instead of push-and-null-check loops.
- Scopes (ch. 5/7): a `Vec<HashMap<…>>` stack plus `fn scoped<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T`, so early returns can't leak bindings (C++ restores by hand and skips it on error paths).
- Host functions (`putchard`, `printd`): `#[unsafe(no_mangle)] pub extern "C" fn` (edition 2024), registered with `ExecutionEngine::add_global_mapping`. A Rust .exe on Windows doesn't export symbols for the JIT to find.