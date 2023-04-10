
/// SST文件又一个个块组成，块中可以保存数据、数据索引、元数据或者元数据索引。
///
/// SST文件的格式:
/// <beginning_of_file>
///     [data block 1]       -- data block 数据区域（保存具体的键值对数据）， 块格式保存
///     [data block 2]       -- 每当 data block 的大小2K的时候，开始创建一个filter
///     ...
///     [data block N]
///     [meta block 1]        -- 元数据区域（保存元数据，如布隆过滤器数据），只有一个 meta block。
///                              不按照块格式保存. 通过 FilterBlockBuilder 构建
///
///     [meta block index]    -- 元数据索引区域， 块格式保存, BlockHandler
///     [data block index]    -- 数据索引区域， 块格式保存, BlockHandler
///     [Footer]              -- 尾部（总大小固定48个字节） @see format#Footer
/// <end_of_file>
///
/// 通过读取 Footer，可以定位到 数据索引区域（data block index）以及元数据索引区域（meta block index）.
/// 通过索引区域后，可以继续定位到具体的数据。
///
pub struct SSTable {
}