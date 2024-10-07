# imlang

The Im programming language

When it can bootstrap, it will be refactored

---

# Supports:

- [x] Variables
- [x] Printing
- [x] Arithmetic
- [x] Parentheses
- [ ] Comments
- [ ] Strings
- [ ] Functions
- [ ] Control Flow
- [ ] Classes
- [ ] Standard Library
- [ ] Error Handling
- [ ] Bootstrapping

---

# Syntax

``` python
x = 5
print x

y = x + 3
print y

print(x + y)
print x - y
print(x * y)
print x / y

z = (x + y) * 2 * (x - (x + y) + (x * y) / y)
print(z)
```

output:

``` python
x = 5
y = 8
(x + y) = 13
(x - y) = -3
(x * y) = 40
(x / y) = 0
z = -78
```