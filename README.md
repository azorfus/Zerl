### Zerl

Zerl is a low-level, statically typed language designed to compile directly to bare-metal RISC-V machine code. It provides a minimal set of constructs — functions, loops, conditionals, and direct memory access — suitable for writing standalone kernels, bootable programs, or embedded applications without relying on external runtimes or standard libraries.

---

### Features

- **Bare-metal by default** — No dependencies, no runtime, no external kernel.

- **Direct RISC-V code generation** — Outputs raw binaries compatible with QEMU or hardware.

- **Explicit control** — Full access to memory, peripherals, and program flow.

- **Minimal core language** — Focused on simplicity: `fn`, `if`, `loop`, and low-level operations.

- **Designed for OS and systems development** — Suitable for bootloaders, monitors, demos, or full custom kernels.

---

### Example Zerl Code

```rust
let u8 UART0 = 0x10000000;

fn putc(c: u8) {
    *(UART0 as *mut u8) = c;
}

fn main() {
    let msg = "Hello, world!\n";
    let mut i = 0;
    loop (msg[i] != 0) {
        putc(msg[i]);
        i = i + 1;
    }
    loop (true) {}
}
```

# TODO List

- [x] **Lexer**
  
  - Tokenize keywords, identifiers, numbers, symbols, strings
  - Handle whitespace, comments, and errors
    
    

- [x] **Parser**
  
  - Build AST for:
    - Functions
    - Variables
    - Conditionals (`if`, `else`)
    - Loops (`loop (condition)`)
    - Expressions and statements
      
      

- [ ] **Semantic Analyzer**
  
  - Type checking (e.g., `u8`, `u16`, `bool`)
  - Detect undeclared variables, type mismatches
  - Enforce return types and scope rules
  - Const/static evaluation (optional)
    
    

- [ ] **Intermediate Repressentation (IR)**
  
  - Define a simple intermediate representation
  - Useful for optimization and codegen clarity
    
    

- [ ] **Code Generator**
  
  - Translate IR to RISC-V assembly
  - Support:
    - Function prologue/epilogue
    - Expression evaluation
    - Branching, loops
    - Memory access (load/store)
    - UART output (MMIO)
      
      

- [ ] **Assembler Output**
  
  - Emit `.text`, `.data`, and labels
  - Output valid RISC-V assembly or binary format
    
    

- [ ] **Linker / Binary Generator**
  
  - Flatten sections to `.bin`
  - Generate bootable layout (entry point, alignment)




