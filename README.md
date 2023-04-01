# LevelDB_Rust

## 仓库
* 开发库： [Gitee/level_db_rust](https://gitee.com/rust_us/level_db_rust)
* 同步只读库： [Github/level_db_rust](https://github.com/rust-us/level_db_rust)

## 介绍

LevelDB for rust.
LevelDB是一款写性能十分优秀的可持久化的KV存储引擎，其实现原理是依据LSM-Tree（Log Structed-Merge Tree）.

## 软件架构

![LevelDB--整体架构](doc/images/LevelDB--整体架构.png)

LevelDB是一款写性能十分优秀的可持久化的KV存储引擎，其实现原理是依据LSM-Tree（Log Structed-Merge Tree）.

LSM tree (log-structured merge-tree) 是一种对写操作非常友好的存储方案。

LSM tree 是许多 KV型或日志型数据库所依赖的核心实现，例如BigTable、HBase、Cassandra、LevelDB、SQLite、RocksDB 等

## 安装教程

1. xxxx

## 使用说明

1. 编译参数
   CORE_DEBUG 默认开启，打印调试信息

在构建正式版本时，用户可以用 RUSTFLAGS 环境变量覆盖以上编译参数。
eg:
```bash 
RUSTFLAGS='--cfg CORE_DEBUG="false"' cargo build --release
```

#### 参与贡献

1.  Fork 本仓库
2.  新建 feat/1.0.0_util_xxx 分支
3.  提交代码
4.  新建 Pull Request

[TODO和分工](doc/TODOList.md)

## 编码和git规范

[编码和git规范](doc/CodeStyle.md)

## RoadMap
### 1.0.0
1.0.0 版本, 完成 util 相关的内容

| 功能模块                          | 完成人             | 进度   |
|-------------------------------|-----------------|------|
| Arena (Memory Management)     | wangboo         | 100% |
| bloom                         | fengyang        | 100% |
| Cache                         | colagy          | 30%  |
| Coding (Primitive Type SerDe) | colagy          | 100% |
| Comparator                    | fengyang        | 100% |
| CRC                           | wangboo、lxd5866 | 100% |
| Env                           | lxd5866         |      |
| filter_policy                 | fengyang        | 100% |
| Hash                          | fengyang        | 100% |
| Histgram                      | kazeseiriou     | 100% |
| loging                        |                 |      |
| MutexLock                     | kazeseiriou     | 100% |
| Random                        | colagy          | 100% |
| Status                        | fengyang        | 100% |
| Slice                         | wangboo         | 100% |

### 1.1.0 版本, 完成基础零部件

| 功能模块                                                                             | 完成人                  | 进度   |
|----------------------------------------------------------------------------------|----------------------|------|
| util.Options(ReadOptions, WriteOptions)                                          | kazeseiriou,wangboo  | 0%   |
| util.ENV(SequentialFile, RandomAccessFile, FileLock)                             | lxd5866              | 0%   |
| util.Logger/Log日志库                                                               | peach                | 50%  |
| table.format(Footer, BlockHandle)                                                | 半支烟                  | 20%  |
| db.dbformat(InternalKeyComparator, InternalFilterPolicy, LookupKey, InternalKey) | 半支烟                  | 20%  |
| db.SkipList                                                                      | wangboo              | 100% |
| table.Iterator(DBIter、EmptyIterator)                                             | kazeseiriou          | 0%   |
| table.Iterator(merger.MergingIterator)                                           | kazeseiriou          | 0%   |
| table.Iterator(TwoLevelIterator)                                                 | kazeseiriou          | 0%   |
| table.Iterator(tabletest.KeyConvertingIterator)                                  | kazeseiriou          | 0%   |
| table.Iterator(dbtest.ModelIter)                                                 | kazeseiriou          | 0%   |
| table.Iterator(Block::Iter)                                                      | fengyang             | 0%   |
| IteratorWrapper                                                                  | kazeseiriou          | 0%   |
| db.MemTable(MemTable, MemTableIterator)                                          | wangboo,tzcyujunyong | 20%  | 
| db.Builder                                                                       | fengyang             | 20%  |
| table.Block                                                                      | fengyang             | 30%  |
| table.BlockBuilder, table.FilterBlockBuilder                                     |                      |      |
| FilterBlock, FilterBlockReader                                                   | fengyang             | 80%  |
| SSTable                                                                          | fengyang             | 0%   |
| table.Table                                                                      | peach,tzcyujunyong   |      |
| db.leveldb_util                                                                  | wangboo              | 0%   |
| db.log_format                                                                    | wangboo              | 90%  |
| db.LogReader                                                                     | wangboo              | 90%  |
| db.LogWriter                                                                     | wangboo              | 90%  |
| db.TableCache                                                                    | colagy               | 10%  |
| LinkedList                                                                       | fengyang             | 60%  |
| db.VersionEdit(Tag, VersionEdit, FileMetaData)                                   | fengyang             | 20%  |
| db.VersionSet(Version, LevelFileNumIterator, SaverState)                         | fengyang             | 20%  |
| WriteBatch                                                                       | tzcyujunyong,wangboo | 50%  |
| table.table_builder                                                              |                      | 30%  |
| db.filename                                                                      |                      |      |
| <website>                                                                        | 半支烟                  | 40%  |





#### 1.1.0 计划
* 完成gitee ->  github  (同步)  主仓库gitee
* 官网交给辉哥, 展示一些要做的内容和产品信息。 2月完成。
* 优先接口对齐, 对齐后再各自写实现, 对齐时间: 1月17号晚, 1月18号下午
* 注释 使用rust标准, 使用中文注释

### 1.2.0
1.2.0 版本, 完成核心组件

| module              | contributor | process |
|---------------------|-------------|---------|
| DB(DBImpl, ModelDB) | none        |         |
| Repairer            | none        |         |
| Snapshot            | none        |         |
| DumpFile            | none        |         |
|                     | none        |         |

   
