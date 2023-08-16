
# get target-features

```sh
rustc --print target-features > target-features.txt
awk -F ' - ' '{gsub(/ /, "", $1); print "\"" $1 "\","}' target-features.txt
```
