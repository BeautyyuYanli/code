# Week 2 Pre-class Exercise

[CS 110L: Safety in Systems Programming](https://web.stanford.edu/class/cs110l/lecture-notes/lecture-03/)

`int new_capacity = vec->capacity * 2;`

The capacity is initially 0.

`vec->data = new_data;`

The old data doesn't be freed.

`free(vec);``free(vec->data);`

Inversed.

```
  int* n = &vec->data[0];
  vec_push(vec, 110);
  printf("%d\n", *n);
```

`[]` accesses to a location may be invaild.

If the vector grows when pushing, the pointer made formerly is invalid. The same as std::vector.

```
  free(vec->data);
  vec_free(vec);
```

The problem is related to the previous function `vec_free`.