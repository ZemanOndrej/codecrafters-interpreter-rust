
### input = 1 + 2 * 3 * 4 + 1; result = 26


1. i = 1; s =(1); input = "+ 2 * 3 * 4 + 1"
2. i = +; s =((1+2)); input = " * 3 * 4 + 1"
3. i = *; s = (((1+2) * 3)); input = " * 4 + 1"

...


### correct

1. i = 1; s =(1); input = "+ 2 * 3 * 4 + 1"
2. i = +; s =(2 * 3); incomplete = 1+ ; input = " * 4 + 1"
3. i = *; s =((2 * 3) * 4); incomplete = 1+ ; input = " + 1"
4. i = *; s = (1 + ((2 * 3) * 4)); input = " + 1"
5. i = +; s = ((1 + ((2 * 3) * 4)) + 1); input = ""
