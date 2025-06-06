### KiB

- 1 KiB is Kibibyte = 1024 bytes 
- 0x1000 = 4 KiB
- 0x8000 = 16 KiB
- 0x = base 16
- 0x1000 = 4096
  - 1 x 16^3 = 4096
  - 0 x 16^2 = 0
  - 0 x 16^1 = 0
  - 0 x 16^0 = 0

### Shift Operator ( << k )

- Moves all bit one place to the left, adding zeros to the right
- Move k bits to the left (is like multiply the value by 2^k)

 Example:
```
a = 0b0010 = 2
b = 0b0010 << 2 = 0b1000
```

### Bitwise OR
 - It compares each bit of two numbers and returns 1 if either bit is 1. Otherwise, it returns 0.

```
  0b1010_0000   (160 in decimal)
| 0b0101_0000   ( 80 in decimal)
-------------
  0b1111_0000   (240 in decimal)
```

### For instructions

### Cycles
This is how many clock cycles (or machine cycles) the CPU takes to execute the instruction.

- 1 cycle = 238 ns = (1/4194304) seg

For LDH [n16], A, it takes 3 cycles.

Each cycle corresponds to a fixed amount of time the CPU needs to process part of the instruction.

The more cycles, the longer the instruction takes.

### Bytes
How many bytes the instruction occupies in memory.

For LDH [n16], A, it’s 2 bytes:

1 byte for the opcode

1 byte for the immediate address (the n16 in this case is actually 8-bit — n8 — so maybe you mean the 8-bit offset to $FF00? Because the LDH uses an 8-bit address, not full 16-bit.)

Important: LDH instructions use an 8-bit immediate address which is offset into the I/O area (0xFF00 + n8).

### PC
The Program Counter (PC) increments by the number of bytes in the instruction to point to the next instruction.

So yes, after executing LDH [n8], A which is 2 bytes, PC increases by 2 (opcode + immediate byte).

PC increment it’s based on instruction length (bytes)

**After a instruction finish his work, then return the amount of cycles (in GB CPU) that take it to complete**