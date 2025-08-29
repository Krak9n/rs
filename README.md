This is my analogue to **ls** command from gnu coreutils. Shows basic information.

> [!Read]
> This thing is made for myself. However, 
if you have any problems feel free to change and/or open pull requests.

<!--Works on both windows and linux.-->
**s** stands for **show**.

### Dependencies
+ cargo

### Installation
After cloning just, run this command
```
# cargo install --path .
```

### Usage:
-------
Basic file showing.
```
rs
```
--------
Showing in the reverse order.
```
rs -r 
```
--------
Shows basic information about a file.
+ size
+ last modified date for a file
+ name
```
rs -i
``` 

