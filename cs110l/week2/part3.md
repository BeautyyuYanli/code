- Example 1:
  
  Once there exist at least one reference, the mutable variable will be immutable temporarily.
  
  ```rust
  fn main() {
      let mut s = String::from("hello");
      let ref1 = &s;
      let ref2 = &ref1;
      let ref3 = &ref2;
      s = String::from("goodbye");
      println!("{}", ref3.to_uppercase());
  }
  ```

- Example 2:
  
  `s` will be destroyed when the function ends, so the returned reference will be invaild.
  
  ```rust
  fn drip_drop() -> &String {
      let s = String::from("hello world!");
      return &s;
  }
  ```

- Example 3:
  
  `v[0]` is owned by the vector, and can't be transferred to `s2`.
  
  ```rust
  fn main() {
      let s1 = String::from("hello");
      let mut v = Vec::new();
      v.push(s1);
      let s2: String = v[0];
      println!("{}", s2);
  }
  ```
