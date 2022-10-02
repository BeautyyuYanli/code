The answer bytes can be found in `./answer/<lv>.txt`.

## Lv 1
Overwrite `0x401976`(the real ret addr) with `0x4017c0`(the func `touch1` addr). 

The address can be examined with `x 0x28+$rsp` when the program is inside the function `getbuf`.