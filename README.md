This is my analogue to **ls** command from gnu coreutils. Shows basic information.
Works on both windows and linux.

**s** stands for **show**.

### Dependencies
+ cargo

### Installation
After cloning just, run this command
```
cargo install --path .
```

### Usage:
Basic file showing.
```
s
```

Showing in the reverse order.
```
s -r 
```

shows basic information about a file.
+ size
+ creation date
+ name
```
s -i
``` 

