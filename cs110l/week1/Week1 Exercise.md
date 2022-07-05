# Week1 Exercise

[CS 110L: Safety in Systems Programming](https://web.stanford.edu/class/cs110l/assignments/week-1-exercises/)

# **Program 1: UPPERCASe**

## **Question 1**

The function `my_strcpy` is user-defined so that the linter doesn't apply the info that "strcpy may produce a no null-terminated string", which is actually made by `my_strcpy`. The data analyzer may find that `dest[i] = src[i]` can be harmful, but in most cases, it's handled by the programmers and safe, so the data analyzer doesn't report it.

## **Question 2**

No. The function `my_strcpy` write to a location that is not allocated by `char uppercase[...]`, and is not guaranteed to be available. But the location is located by the pointer `dest` so that valgrind doesn't understand that it may be not available.

## **Question 3**

When defining `char uppercase[...]`, the sanitizer recorded the size of the initialized stack memory. So when writing `\0` to the location outside the char array, the sanitizer will notice that the offset exceeds the size of the initialized stack memory.

# **Program 2: Linked~~~Lists–**

## **Question 4**

Because `kNumElements` is defined as a variable, clang-tidy thinks it may cause the loop not to be executed, and further causes access to NULL. If we defined `kNumElements` as constant, then the false positive won't happen. This may be a result of replacing the constant in the wey like a macro. 

## **Question 5**

The sanitizer funded the memory leak by tracking things created by `malloc`.

# **Program 3: Parsing and Early Returns**

## Question 6

clang-tidy detected the memory leak by analyzing the data flow.

## **Question 7**

1. Because the input doesn’t match the case that ‘]’ doesn’t exist and the memory leak happens.
2. I tried to use “hi [hello world!” but failed.

## **Question 8**

A simple one is `[[`. 

I noticed that sanitizer always report at the second case of ']' doesn't exist. So I run `parse(argv[1])` twice with such a case of input, and the sanitizer does report.

clang version 13.0.1

# **Program 4: Fibonacci**

## **Question 9**

It’s just wrong to the users, but not wrong to the computer. The computer can generate the array with no error.