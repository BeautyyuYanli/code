## phase 1

```asm
0000000000400ee0 <phase_1>:
  400ee0:    48 83 ec 08              sub    $0x8,%rsp
  400ee4:    be 00 24 40 00           mov    $0x402400,%esi
  400ee9:    e8 4a 04 00 00           call   401338 <strings_not_equal>
  400eee:    85 c0                    test   %eax,%eax
  400ef0:    74 05                    je     400ef7 <phase_1+0x17>
  400ef2:    e8 43 05 00 00           call   40143a <explode_bomb>
  400ef7:    48 83 c4 08              add    $0x8,%rsp
  400efb:    c3                       ret    
```

Inspecting <strings_not_equal>

```asm
0000000000401338 <strings_not_equal>:
  401338:    41 54                    push   %r12
  40133a:    55                       push   %rbp
  40133b:    53                       push   %rbx
  40133c:    48 89 fb                 mov    %rdi,%rbx
  40133f:    48 89 f5                 mov    %rsi,%rbp
  401342:    e8 d4 ff ff ff           call   40131b <string_length>
  401347:    41 89 c4                 mov    %eax,%r12d
  40134a:    48 89 ef                 mov    %rbp,%rdi
  40134d:    e8 c9 ff ff ff           call   40131b <string_length>
  401352:    ba 01 00 00 00           mov    $0x1,%edx
  401357:    41 39 c4                 cmp    %eax,%r12d
  40135a:    75 3f                    jne    40139b <strings_not_equal+0x63>
  40135c:    0f b6 03                 movzbl (%rbx),%eax
  40135f:    84 c0                    test   %al,%al
  401361:    74 25                    je     401388 <strings_not_equal+0x50>
  401363:    3a 45 00                 cmp    0x0(%rbp),%al
  401366:    74 0a                    je     401372 <strings_not_equal+0x3a>
  401368:    eb 25                    jmp    40138f <strings_not_equal+0x57>
  40136a:    3a 45 00                 cmp    0x0(%rbp),%al
  40136d:    0f 1f 00                 nopl   (%rax)
  401370:    75 24                    jne    401396 <strings_not_equal+0x5e>
  401372:    48 83 c3 01              add    $0x1,%rbx
  401376:    48 83 c5 01              add    $0x1,%rbp
  40137a:    0f b6 03                 movzbl (%rbx),%eax
  40137d:    84 c0                    test   %al,%al
  40137f:    75 e9                    jne    40136a <strings_not_equal+0x32>
  401381:    ba 00 00 00 00           mov    $0x0,%edx
  401386:    eb 13                    jmp    40139b <strings_not_equal+0x63>
  401388:    ba 00 00 00 00           mov    $0x0,%edx
  40138d:    eb 0c                    jmp    40139b <strings_not_equal+0x63>
  40138f:    ba 01 00 00 00           mov    $0x1,%edx
  401394:    eb 05                    jmp    40139b <strings_not_equal+0x63>
  401396:    ba 01 00 00 00           mov    $0x1,%edx
  40139b:    89 d0                    mov    %edx,%eax
  40139d:    5b                       pop    %rbx
  40139e:    5d                       pop    %rbp
  40139f:    41 5c                    pop    %r12
  4013a1:    c3                       ret    
```

Notice:

```asm
  40133c:    48 89 fb                 mov    %rdi,%rbx
  40133f:    48 89 f5                 mov    %rsi,%rbp
...
  401352:    ba 01 00 00 00           mov    $0x1,%edx
  401357:    41 39 c4                 cmp    %eax,%r12d
  40135a:    75 3f                    jne    40139b <strings_not_equal+0x63>
...
  40139b:    89 d0                    mov    %edx,%eax
```

The function compares strings from `%rdi` and `%rsi`. `%eax,%r12d` are length of strings. If they are not equal then jump to `40139b`, which return `%edx = 1`.

So the function return 1 if the strings are not equal.

Back to <phase_1>, the test instruction indicates "call <explode_bomb> if <strings_not_equal> doesn't return 0". `mov    $0x402400,%esi` indicates `0x402400` is one of the parameters passed to <strings_not_equal>. The value should be a `char *`, so we use `(gdb) x/s 0x402400` to print the mem data as a string, which results `Border relations with Canada have never been better.`.

## phase 2

```asm
0000000000400efc <phase_2>:
  400efc:    55                       push   %rbp                        
  400efd:    53                       push   %rbx                        
  400efe:    48 83 ec 28              sub    $0x28,%rsp                        
  400f02:    48 89 e6                 mov    %rsp,%rsi                        
  400f05:    e8 52 05 00 00           call   40145c <read_six_numbers>                        
  400f0a:    83 3c 24 01              cmpl   $0x1,(%rsp)                        
  400f0e:    74 20                    je     400f30 <phase_2+0x34># >======|              
  400f10:    e8 25 05 00 00           call   40143a <explode_bomb>#        |              
  400f15:    eb 19                    jmp    400f30 <phase_2+0x34># >======|              
                                                                  #        |
  400f17:    8b 43 fc                 mov    -0x4(%rbx),%eax      # <~~~~~~|~~|      
  400f1a:    01 c0                    add    %eax,%eax            #        |  |
  400f1c:    39 03                    cmp    %eax,(%rbx)          #        |  |  
  400f1e:    74 05                    je     400f25 <phase_2+0x29>#        |  |            
  400f20:    e8 15 05 00 00           call   40143a <explode_bomb>#        |  |            
  400f25:    48 83 c3 04              add    $0x4,%rbx            #        |  |
  400f29:    48 39 eb                 cmp    %rbp,%rbx            #        |  |
  400f2c:    75 e9                    jne    400f17 <phase_2+0x1b># >~~~~~~|~~|            
  400f2e:    eb 0c                    jmp    400f3c <phase_2+0x40># >--|   |  |            
                                                                  #    |   |  |
  400f30:    48 8d 5c 24 04           lea    0x4(%rsp),%rbx       # <======|  |     
  400f35:    48 8d 6c 24 18           lea    0x18(%rsp),%rbp      #    |      |      
  400f3a:    eb db                    jmp    400f17 <phase_2+0x1b># >~~~~~~~~~|                      
                                                                  #    |
  400f3c:    48 83 c4 28              add    $0x28,%rsp           # <--|        
  400f40:    5b                       pop    %rbx                        
  400f41:    5d                       pop    %rbp                        
  400f42:    c3                       ret    

000000000040145c <read_six_numbers>:
  40145c:    48 83 ec 18              sub    $0x18,%rsp
  401460:    48 89 f2                 mov    %rsi,%rdx        # arg 3 = %rsi
  401463:    48 8d 4e 04              lea    0x4(%rsi),%rcx   # arg 4 = %rsi+4
  401467:    48 8d 46 14              lea    0x14(%rsi),%rax
  40146b:    48 89 44 24 08           mov    %rax,0x8(%rsp)   # arg 8 = %rsi+20
  401470:    48 8d 46 10              lea    0x10(%rsi),%rax
  401474:    48 89 04 24              mov    %rax,(%rsp)      # arg 7 = %rsi+16
  401478:    4c 8d 4e 0c              lea    0xc(%rsi),%r9    # arg 6 = %rsi+12
  40147c:    4c 8d 46 08              lea    0x8(%rsi),%r8    # arg 5 = %rsi+8
  401480:    be c3 25 40 00           mov    $0x4025c3,%esi   # arg 2 = 0x4025c3
  401485:    b8 00 00 00 00           mov    $0x0,%eax
  40148a:    e8 61 f7 ff ff           call   400bf0 <__isoc99_sscanf@plt> # arg 1: user input
  40148f:    83 f8 05                 cmp    $0x5,%eax
  401492:    7f 05                    jg     401499 <read_six_numbers+0x3d>
  401494:    e8 a1 ff ff ff           call   40143a <explode_bomb>
  401499:    48 83 c4 18              add    $0x18,%rsp
  40149d:    c3                       ret    
```

The function <phase_2> reads six int numbers from `%rsi` and stores them from `(%rsp)` to `20(%rsp)`. If the input is not six int numbers, then it calls <explode_bomb> function.

Then it checks if the first number is equal to 1. If it isn't, then it calls <explode_bomb> function. 

Else it checks if every number is 2x of the previous one.

So the answer is `1 2 4 8 16 32`.

## phase 3
```asm
0000000000400f43 <phase_3>:
  400f43:	48 83 ec 18          	sub    $0x18,%rsp
  400f47:	48 8d 4c 24 0c       	lea    0xc(%rsp),%rcx  # arg 4 = %rsp + 12
  400f4c:	48 8d 54 24 08       	lea    0x8(%rsp),%rdx  # arg 3 = %rsp + 8
  400f51:	be cf 25 40 00       	mov    $0x4025cf,%esi  # arg 2 = "%d %d"
  400f56:	b8 00 00 00 00       	mov    $0x0,%eax
  400f5b:	e8 90 fc ff ff       	call   400bf0 <__isoc99_sscanf@plt>
  400f60:	83 f8 01             	cmp    $0x1,%eax
  400f63:	7f 05                	jg     400f6a <phase_3+0x27>
  400f65:	e8 d0 04 00 00       	call   40143a <explode_bomb>
  400f6a:	83 7c 24 08 07       	cmpl   $0x7,0x8(%rsp)
  400f6f:	77 3c                	ja     400fad <phase_3+0x6a> # first int (denoted as i1) should <= 7 (unsigned)
  400f71:	8b 44 24 08          	mov    0x8(%rsp),%eax
  400f75:	ff 24 c5 70 24 40 00 	jmp    *0x402470(,%rax,8)    # jump to 400f7c + 8*i1
```

Firstly it reads two int, denoted as `i1` and `i2` by us. Then it enters a switch statement depending on the value of `i1`.

Jump Table for 400f7c + 8*i1

| i1 |  *DEST   |
|----|----------|
| 0  | 0x400f7c |
| 1  | 0x400fb9 |
| 2  | 0x400f83 |
| 3  | 0x400f8a |
| 4  | 0x400f91 |
| 5  | 0x400f98 |
| 6  | 0x400f9f |
| 7  | 0x400fa6 |

```asm
# 0
  400f7c:	b8 cf 00 00 00       	mov    $0xcf,%eax  # 207
  400f81:	eb 3b                	jmp    400fbe <phase_3+0x7b>
# 2
  400f83:	b8 c3 02 00 00       	mov    $0x2c3,%eax # 707
  400f88:	eb 34                	jmp    400fbe <phase_3+0x7b>
# 3
  400f8a:	b8 00 01 00 00       	mov    $0x100,%eax # 256
  400f8f:	eb 2d                	jmp    400fbe <phase_3+0x7b>
# 4
  400f91:	b8 85 01 00 00       	mov    $0x185,%eax # 389
  400f96:	eb 26                	jmp    400fbe <phase_3+0x7b>
# 5
  400f98:	b8 ce 00 00 00       	mov    $0xce,%eax  # 206
  400f9d:	eb 1f                	jmp    400fbe <phase_3+0x7b>
# 6
  400f9f:	b8 aa 02 00 00       	mov    $0x2aa,%eax # 682
  400fa4:	eb 18                	jmp    400fbe <phase_3+0x7b>
# 7
  400fa6:	b8 47 01 00 00       	mov    $0x147,%eax # 327
  400fab:	eb 11                	jmp    400fbe <phase_3+0x7b>
# > 7
  400fad:	e8 88 04 00 00       	call   40143a <explode_bomb>
  400fb2:	b8 00 00 00 00       	mov    $0x0,%eax   # 0
  400fb7:	eb 05                	jmp    400fbe <phase_3+0x7b>
# 1
  400fb9:	b8 37 01 00 00       	mov    $0x137,%eax # 311

  400fbe:	3b 44 24 0c          	cmp    0xc(%rsp),%eax
  400fc2:	74 05                	je     400fc9 <phase_3+0x86> # i2 should equal to %eax
  400fc4:	e8 71 04 00 00       	call   40143a <explode_bomb>
  400fc9:	48 83 c4 18          	add    $0x18,%rsp
  400fcd:	c3                   	ret    
```

Then it checks if `i2` matchs the counterpart switched value. The answer can be any of the following:
- `0 207`
- `1 311`
- `2 707`
- `3 256`
- `4 389`
- `5 206`
- `6 682`
- `7 327`