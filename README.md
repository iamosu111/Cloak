#### Input format

```
block_id [address] {in/out, lambda, amount, timestamp}
```

For example

```
1 [muhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4, 11, 1571443461}
1 [mwhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4,12, 1571443461}
1 [mvbnrCX3bg1cDRUu8pkecrvP6vQkSLDSou] {out, 4,10, 1571443461}
1 [muhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4, 3, 1571443461}
1 [mwhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4, 7, 1571443461}
1 [mvbnrCX3bg1cDRUu8pkecrvP6vQkSLDSou] {out, 4, 6, 1571443461}
1 [muhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4, 5, 1571443461}
1 [mwhtvdmsnbQEPFuEmxcChX58fGvXaaUoVt] {in, 4, 4, 1571443461}
```

### Decompose

##### decompose.json

```
[
{ "query_position": 2,
"coefficient_p": 4,
"coefficient_N": 8, "coefficient_lambda": 4 }
]
```

position表示查询数据的位置，2表示查询第2个数据

p表示分片个数，4表示分4片

N表示总数据个数

lambda表示转换成二进制后的字节数，如0111即为4



#### Query

##### query.json

```
{ "blk_height": 1,
"query_slice": [[0,1,1,0,0,0,0,0,0,1,1,0,1,0,0,1,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0],[1,1,0,0,1,1,0,0],[0,1,1,0,1,1,0,0],[1,0,1,0,0,0,0,0],[1,1,1,1,0,1,1,1],[1,1,1,1,0,1,1,0],[0,1,0,1,1,0,1,1],[0,0,0,1,1,0,1,1],[1,0,0,1,1,0,0,0]],
"coefficient_p": 4, "coefficient_lambda": 4 }
```

query_slice表示decompose后的某个分片，每个[[ ]]内为一个分片



#### aggregate

##### aggregate.json

```
{ "result_slice": [[1,0,1,1],[0,1,0,1],[1,0,0,1],[1,0,1,1]], "len": 4}
```

四个分片会得到四个答案，将4个答案分片输入 聚合后获得最后结果