/* 
 * CS:APP Data Lab 
 * 
 * <Please put your name and userid here>
 * 
 * bits.c - Source file with your solutions to the Lab.
 *          This is the file you will hand in to your instructor.
 *
 * WARNING: Do not include the <stdio.h> header; it confuses the dlc
 * compiler. You can still use printf for debugging without including
 * <stdio.h>, although you might get a compiler warning. In general,
 * it's not good practice to ignore compiler warnings, but in this
 * case it's OK.  
 */

#if 0
/*
 * Instructions to Students:
 *
 * STEP 1: Read the following instructions carefully.
 */

You will provide your solution to the Data Lab by
editing the collection of functions in this source file.

INTEGER CODING RULES:
 
  Replace the "return" statement in each function with one
  or more lines of C code that implements the function. Your code 
  must conform to the following style:
 
  int Funct(arg1, arg2, ...) {
      /* brief description of how your implementation works */
      int var1 = Expr1;
      ...
      int varM = ExprM;

      varJ = ExprJ;
      ...
      varN = ExprN;
      return ExprR;
  }

  Each "Expr" is an expression using ONLY the following:
  1. Integer constants 0 through 255 (0xFF), inclusive. You are
      not allowed to use big constants such as 0xffffffff.
  2. Function arguments and local variables (no global variables).
  3. Unary integer operations ! ~
  4. Binary integer operations & ^ | + << >>
    
  Some of the problems restrict the set of allowed operators even further.
  Each "Expr" may consist of multiple operators. You are not restricted to
  one operator per line.

  You are expressly forbidden to:
  1. Use any control constructs such as if, do, while, for, switch, etc.
  2. Define or use any macros.
  3. Define any additional functions in this file.
  4. Call any functions.
  5. Use any other operations, such as &&, ||, -, or ?:
  6. Use any form of casting.
  7. Use any data type other than int.  This implies that you
     cannot use arrays, structs, or unions.

 
  You may assume that your machine:
  1. Uses 2s complement, 32-bit representations of integers.
  2. Performs right shifts arithmetically.
  3. Has unpredictable behavior when shifting if the shift amount
     is less than 0 or greater than 31.


EXAMPLES OF ACCEPTABLE CODING STYLE:
  /*
   * pow2plus1 - returns 2^x + 1, where 0 <= x <= 31
   */
  int pow2plus1(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     return (1 << x) + 1;
  }

  /*
   * pow2plus4 - returns 2^x + 4, where 0 <= x <= 31
   */
  int pow2plus4(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     int result = (1 << x);
     result += 4;
     return result;
  }

FLOATING POINT CODING RULES

For the problems that require you to implement floating-point operations,
the coding rules are less strict.  You are allowed to use looping and
conditional control.  You are allowed to use both ints and unsigneds.
You can use arbitrary integer and unsigned constants. You can use any arithmetic,
logical, or comparison operations on int or unsigned data.

You are expressly forbidden to:
  1. Define or use any macros.
  2. Define any additional functions in this file.
  3. Call any functions.
  4. Use any form of casting.
  5. Use any data type other than int or unsigned.  This means that you
     cannot use arrays, structs, or unions.
  6. Use any floating point data types, operations, or constants.


NOTES:
  1. Use the dlc (data lab checker) compiler (described in the handout) to 
     check the legality of your solutions.
  2. Each function has a maximum number of operations (integer, logical,
     or comparison) that you are allowed to use for your implementation
     of the function.  The max operator count is checked by dlc.
     Note that assignment ('=') is not counted; you may use as many of
     these as you want without penalty.
  3. Use the btest test harness to check your functions for correctness.
  4. Use the BDD checker to formally verify your functions
  5. The maximum number of ops for each function is given in the
     header comment for each function. If there are any inconsistencies 
     between the maximum ops in the writeup and in this file, consider
     this file the authoritative source.

/*
 * STEP 2: Modify the following functions according the coding rules.
 * 
 *   IMPORTANT. TO AVOID GRADING SURPRISES:
 *   1. Use the dlc compiler to check that your solutions conform
 *      to the coding rules.
 *   2. Use the BDD checker to formally verify that your solutions produce 
 *      the correct answers.
 */


#endif
//1
/* 
 * bitXor - x^y using only ~ and & 
 *   Example: bitXor(4, 5) = 1
 *   Legal ops: ~ &
 *   Max ops: 14
 *   Rating: 1
 */
int bitXor(int x, int y) {
  return ~(~(~x & y) & ~(x & ~y));
}
/* 
 * tmin - return minimum two's complement integer 
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 4
 *   Rating: 1
 */
int tmin(void) {
  return 1<<31;
}
//2
/*
 * isTmax - returns 1 if x is the maximum, two's complement number,
 *     and 0 otherwise 
 *   Legal ops: ! ~ & ^ | +
 *   Max ops: 10
 *   Rating: 1
 */
int isTmax(int x) {
  return !((!(~x)) | ((x+1) ^ ~x));
}
/* 
 * allOddBits - return 1 if all odd-numbered bits in word set to 1
 *   where bits are numbered from 0 (least significant) to 31 (most significant)
 *   Examples allOddBits(0xFFFFFFFD) = 0, allOddBits(0xAAAAAAAA) = 1
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 2
 */
int allOddBits(int x) {
  int mask = 0xAA;
  mask = mask << 8 | mask;
  mask = mask << 16 | mask;
  return !((x & mask) ^ mask);
}
/* 
 * negate - return -x 
 *   Example: negate(1) = -1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 5
 *   Rating: 2
 */
int negate(int x) {
  return ~x+1;
}
//3
/* 
 * isAsciiDigit - return 1 if 0x30 <= x <= 0x39 (ASCII codes for characters '0' to '9')
 *   Example: isAsciiDigit(0x35) = 1.
 *            isAsciiDigit(0x3a) = 0.
 *            isAsciiDigit(0x05) = 0.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 15
 *   Rating: 3
 */
int isAsciiDigit(int x) {
  // 0011 0...
  // 0011 100.
  return (!((x >> 3) ^ 0x6) | !((x>>1) ^ 0x1C));
}
/* 
 * conditional - same as x ? y : z 
 *   Example: conditional(2,4,5) = 4
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 16
 *   Rating: 3
 */
int conditional(int x, int y, int z) {
  // x -> x==0 or x==0xFFFFFFFF(-1)
  x = ~(!x)+1;
  return (~x&y) | (x&z);
}
/* 
 * isLessOrEqual - if x <= y  then return 1, else return 0 
 *   Example: isLessOrEqual(4,5) = 1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 24
 *   Rating: 3
 */
int isLessOrEqual(int x, int y) {
  int xn = (x >> 31);
  int a = xn ^ (y >> 31);
  int b = xn;
  int c = !((y + (~x+1)) >> 31);
  a = ~a + 1;
  return (a&b) | (~a&c);
}
//4
/* 
 * logicalNeg - implement the ! operator, using all of 
 *              the legal operators except !
 *   Examples: logicalNeg(3) = 0, logicalNeg(0) = 1
 *   Legal ops: ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 4 
 */
int logicalNeg(int x) {
  x = x | (x >> 16);
  x = x | (x >> 8);
  x = x | (x >> 4);
  x = x | (x >> 2);
  x = x | (x >> 1);
  return 1 & ~x;
}
/* howManyBits - return the minimum number of bits required to represent x in
 *             two's complement
 *  Examples: howManyBits(12) = 5
 *            howManyBits(298) = 10
 *            howManyBits(-5) = 4
 *            howManyBits(0)  = 1
 *            howManyBits(-1) = 1
 *            howManyBits(0x80000000) = 32
 *  Legal ops: ! ~ & ^ | + << >>
 *  Max ops: 90
 *  Rating: 4
 */
int howManyBits(int x) {
  int in, a0, a1, a2, a3, a4;
  // set the (MSB - 1) to 1
  in = x ^ (x >> 1);
  // set [MSB - 1, 0] to 1..1
  in = in | (in >> 1);
  in = in | (in >> 2);
  in = in | (in >> 4);
  in = in | (in >> 8);
  in = in | (in >> 16);
  // set (MSB) to 1
  in = in + 1;
  // 32-5 decoder
/*
a0 a1 a2 a3 a4 in
0  0  0  0  0  0
1  0  0  0  0  1
0  1  0  0  0  2
1  1  0  0  0  3
0  0  1  0  0  4
1  0  1  0  0  5
0  1  1  0  0  6
1  1  1  0  0  7
0  0  0  1  0  8
1  0  0  1  0  9
0  1  0  1  0  10
1  1  0  1  0  11
0  0  1  1  0  12
1  0  1  1  0  13
0  1  1  1  0  14
1  1  1  1  0  15
0  0  0  0  1  16
1  0  0  0  1  17
0  1  0  0  1  18
1  1  0  0  1  19
0  0  1  0  1  20
1  0  1  0  1  21
0  1  1  0  1  22
1  1  1  0  1  23
0  0  0  1  1  24
1  0  0  1  1  25
0  1  0  1  1  26
1  1  0  1  1  27
0  0  1  1  1  28
1  0  1  1  1  29
0  1  1  1  1  30
1  1  1  1  1  31
*/
// a0:
// 1 3 5 7 9 11 13 15 17 19 21 23 25 27 29 31

  a0 = in | (in >> 16);
  a0 = a0 | (a0 >> 8);
  a0 = a0 | (a0 >> 4);
  a0 = a0 | (a0 >> 2);
  a0 = a0 >> 1;
  a0 = a0 & 1;

// a1:
// 2 3 6 7 10 11 14 15 18 19 22 23 26 27 30 31

  a1 = in | (in >> 16);
  a1 = a1 | (a1 >> 8);
  a1 = a1 | (a1 >> 4);
  a1 = a1 >> 2;
  a1 = a1 | (a1 >> 1);
  a1 = (a1 & 1) << 1;

// a2:
// 4 5 6 7 12 13 14 15 20 21 22 23 28 29 30 31

  a2 = in | (in >> 16);
  a2 = a2 | (a2 >> 8);
  a2 = a2 >> 4;
  a2 = a2 | (a2 >> 2);
  a2 = a2 | (a2 >> 1);
  a2 = (a2 & 1) << 2;

// a3:
// 8 9 10 11 12 13 14 15 24 25 26 27 28 29 30 31

  a3 = in | (in >> 16);
  a3 = a3 >> 8;
  a3 = a3 | (a3 >> 4);
  a3 = a3 | (a3 >> 2);
  a3 = a3 | (a3 >> 1);
  a3 = (a3 & 1) << 3;

// a4:
// 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

  a4 = (in >> 16);
  a4 = a4 | (a4 >> 8);
  a4 = a4 | (a4 >> 4);
  a4 = a4 | (a4 >> 2);
  a4 = a4 | (a4 >> 1);
  a4 = (a4 & 1) << 4;
  return 1 + (a0 | a1 | a2 | a3 | a4);
}
//float
/* 
 * floatScale2 - Return bit-level equivalent of expression 2*f for
 *   floating point argument f.
 *   Both the argument and result are passed as unsigned int's, but
 *   they are to be interpreted as the bit-level representation of
 *   single-precision floating point values.
 *   When argument is NaN, return argument
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
unsigned floatScale2(unsigned uf) {
  unsigned sign_ori, exp, frac;
  sign_ori = uf >> 31 << 31;
  exp = uf << 1 >> 24;
  frac = uf << 9 >> 9;
  // check if NaN or INF
  if (exp == 0xFF) {
      return uf;
  }
  // check if into INF
  if (exp == 0xFE) {
      return sign_ori | (((1 << 8) - 1) << 23);
  }
  // check if denormalized
  if (exp == 0) {
    // check if denormalized into normalized
    frac = frac << 10 >> 9;
    if (frac >> 21) {
      return sign_ori | (0x1 << 23) | frac;
    }
    else {
      return sign_ori | (0x0 << 23) | frac;
    }
  }
  return sign_ori | ((exp + 1) << 23) | frac;
}
/* 
 * floatFloat2Int - Return bit-level equivalent of expression (int) f
 *   for floating point argument f.
 *   Argument is passed as unsigned int, but
 *   it is to be interpreted as the bit-level representation of a
 *   single-precision floating point value.
 *   Anything out of range (including NaN and infinity) should return
 *   0x80000000u.
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
int floatFloat2Int(unsigned uf) {
  int S, E, M;
  S = (uf >> 31) ? (-1) : 1;
  E = (uf << 1 >> 24) - 127;
  M = uf << 9 >> 9;
  // check left-shift or right-shift
  if (E < 0) {
    return 0;
  }
  else if (E >= 23) {
    // left-shift: check overflow
    // also check if INF or NaN
    if (E > 30) {
      return 0x80000000u;
    }
    else return ((M | (1 << 23)) << (E - 23)) * S;
  }
  else {
    return ((M | (1 << 23)) >> (23 - E)) * S;
  }
}
/* 
 * floatPower2 - Return bit-level equivalent of the expression 2.0^x
 *   (2.0 raised to the power x) for any 32-bit integer x.
 *
 *   The unsigned value that is returned should have the identical bit
 *   representation as the single-precision floating-point number 2.0^x.
 *   If the result is too small to be represented as a denorm, return
 *   0. If too large, return +INF.
 * 
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. Also if, while 
 *   Max ops: 30 
 *   Rating: 4
 */
unsigned floatPower2(int x) {
  if (x >= 128) {
    return 0x7F800000;
  }
  else if (x > -127) {
    return (x + 127) << 23;
  }
  else if (x >= -149) {
    return 1 << (x + 129);
  }
  else {
    return 0;
  }
}
