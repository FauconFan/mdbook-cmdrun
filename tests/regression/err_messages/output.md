# Error Messages

The output of a helpful command that returns 1 and I would still like its output.
**cmdrun error**: The following command returned a nonzero exit code (1):

 $ diff a.rs b.rs 

If you don't consider it a failure, consider making it return 0 instead.
For example, by appending ` || true`.

stdout (what would be put into book):
```
2c2
<     println!("I'm from `a.rs`");
---
>     println!("I'm from `b.rs`");

```
stderr (helpful for debugging):
```

```
The output from a simple typo that I need to correct.
**cmdrun error**: The following command returned a nonzero exit code (127):

 $ eco simple typo 

If you don't consider it a failure, consider making it return 0 instead.
For example, by appending ` || true`.

stdout (what would be put into book):
```

```
stderr (helpful for debugging):
```
sh: 1: eco: not found

```