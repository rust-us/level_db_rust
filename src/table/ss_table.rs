
/// SST文件的格式:
/// <beginning_of_file>
///     [data block 1]
///     [data block 2]
///     ...
///     [data block N]
///     [meta block 1]        -- 只有一个 meta block
///     [meta block index]
///     [data block index]
///     [Footer]
/// <end_of_file>
///
/// 一般而言，虽然SST文件里面声称是支持多个meta block的，但是实际上，也只有一个meta block。
/// 此外，会在每当data block的大小2K的时候(见 FilterBlock.rs)，开始创建一个filter。
pub struct SSTable {

}