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