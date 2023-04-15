use std::io::Write;
use std::sync::Arc;
use crate::debug;
use crate::traits::filter_policy_trait::{FilterPolicy, FilterPolicyPtr};
use crate::util::coding::Encoder;
use crate::util::slice::Slice;

use crate::util::Result;

// 对2K取2的对数，也就是得到11
const FILTER_BASE_LG: usize = 11;

// 在每当data block的大小2K的时候(FILTER_BASE的值)，开始创建一个filter
// Generate new filter every 2KB of data
const FILTER_BASE: usize = 1 << FILTER_BASE_LG;

///
/// meta block 构建器
/// FilterBlock，实质上就是SST文件里面的 meta block
///
pub trait FilterBlock {
    fn new_with_policy(policy: FilterPolicyPtr) -> Self;

    ///
    /// 构造一个  FilterBlockBuilder， 分配初始化容量大小
    ///
    /// # Arguments
    ///
    /// * `policy`:
    /// * `capacity`: 初始化容量
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn new_with_policy_capacity(policy: FilterPolicyPtr, capacity: usize) -> Self;

    /// 设置block的起始位置
    ///
    /// # Arguments
    ///
    /// * `_block_offset`: filter block的 偏移量. 当给定block_offset的时候。需要创建的filter的数目也就确定了。
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// filter_block_builder.start_block(1024_u64);
    /// ```
    fn start_block(&mut self, block_offset: u64);

    fn add_key_from_str(&mut self, key: &str);

    /// 添加key到builder
    ///
    /// # Arguments
    ///
    /// * `_key`: 键
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn add_key(&mut self, key: &Slice);

    /// 构造filterBlock
    ///
    /// # Examples
    ///
    /// ```
    /// filter_block_builder.finish();
    /// ```
    fn finish(&mut self) -> Result<Slice>;

    fn get_policy(&self) -> FilterPolicyPtr;

    fn get_keys(&self) -> Vec<u8>;

    fn get_start(&self) -> Vec<usize>;

    fn get_result(&self) -> Vec<u8>;

    fn get_tmp_keys(&self) -> Vec<Slice>;

    fn get_tmp_filter_offsets(&self) -> Vec<u32>;
}

/// SSTable 文件里面的 meta block 构建器, 按内存里面指定的格式整理在内存中
pub struct FilterBlockBuilder {
    // 指向一个具体的filter_policy
    policy: FilterPolicyPtr,

    // 包含了所有展开的keys。并且这些所有的keys都是存放在一起的。(通过 AddKey 达到这个目的)
    keys: Vec<u8>,
    // 记录当前这个key在keys_里面的offset
    start: Vec<usize>,

    // Filter data computed so far
    // 用result_来记录所有的输入.
    // result_变量就是表示的是一个filter计算之后的输出。
    // 比如 BloomFilter 经过各种key计算之后，可能会得到一个 filter_str。这个 filter_str 就是放到result里面。
    result: Vec<u8>,

    // policy_->CreateFilter() argument
    tmp_keys: Vec<Slice>,
    // 里面的每个元素就是用来记录每个filter内容的offset
    filter_offsets: Vec<u32>,
}

pub struct FilterBlockReader {
    policy: FilterPolicyPtr,
    // Pointer to filter data (at block-start)
    data: Vec<u32>,
    // Pointer to beginning of offset array (at block-end)
    offset: Vec<u32>,
    // Number of entries in offset array
    num: usize,
    // Encoding parameter (see kFilterBaseLg in .cc file)
    base_lg: usize,
}

impl FilterBlock for FilterBlockBuilder {
    fn new_with_policy(policy: FilterPolicyPtr) -> Self {
        FilterBlock::new_with_policy_capacity(policy, 64)
    }

    fn new_with_policy_capacity(policy: FilterPolicyPtr, capacity: usize) -> Self {
        let keys: Vec<u8> = Vec::with_capacity(capacity);
        let start: Vec<usize> = Vec::with_capacity(capacity);
        let result: Vec<u8> = Vec::with_capacity(capacity);
        let tmp_keys: Vec<Slice> = vec![];
        let filter_offsets: Vec<u32> = vec![];

        Self {
            policy,
            keys,
            start,
            result,
            tmp_keys,
            filter_offsets,
        }
    }

    fn start_block(&mut self, block_offset: u64) {
        // 计算出需要创建的filter的总数目. filters_number ==> filter_index
        let filters_number = block_offset / (FILTER_BASE as u64);

        let len = self.filter_offsets.len() as u64;
        assert!(filters_number >= len);

        // 当已经生成的filter的数目小于需要生成的filter的总数时，那么就继续创建filter。
        while filters_number > len {
            self.generate_new_filter();
        }
    }

    fn add_key_from_str(&mut self, key: &str) {
        self.add_key(&Slice::from(key))
    }

    fn add_key(&mut self, key: &Slice) {
        self.start.push(self.keys.len());
        self.keys.write(key.as_str().as_bytes()).expect("add_key error!");
    }

    fn finish(&mut self) -> Result<Slice> {
        if self.start.len() != 0 {
            self.generate_new_filter();
        }

        // Append array of per-filter offsets
        let array_offset = self.result.len() as u32;

        // 当前需要写入的位置。result 中可能存在数据，因此为 offset ==> self.result.len()  的位置
        let dst: &mut Vec<u8> = &mut self.result;
        // let mut dst_append = self.result.as_mut_slice();
        let mut encoder = Encoder::with_vec(dst);
        for i in 0..self.filter_offsets.len() {
            encoder.put_fixed32(self.filter_offsets[i])?;
        }
        encoder.put_fixed32(array_offset)?;

        // Save encoding parameter in result
        encoder.put_varint64(FILTER_BASE_LG as u64)?;

        Ok(Slice::from_buf(&self.result))
    }

    fn get_policy(&self) -> FilterPolicyPtr {
        self.policy.clone()
    }

    fn get_keys(&self) -> Vec<u8> {
        self.keys.to_vec()
    }

    fn get_start(&self) -> Vec<usize> {
        self.start.to_vec()
    }

    fn get_result(&self) -> Vec<u8> {
        self.result.to_vec()
    }

    fn get_tmp_keys(&self) -> Vec<Slice> {
        self.tmp_keys.to_vec()
    }

    fn get_tmp_filter_offsets(&self) -> Vec<u32> {
        self.filter_offsets.to_vec()
    }
}

impl FilterBlockBuilder {
    /// 创建新的 filter
    fn generate_new_filter(&mut self) {
        // 拿到key的数目
        let num_keys = self.start.len();

        // 如果当前key数目还是0
        if num_keys == 0 {
            // 如果key数目为0，这里应该是表示要新生成一个filter.  这时应该是重新记录下offset了
            // Fast path if there are no keys for this filter
            self.filter_offsets.push(self.result.len() as u32);
            return;
        }

        /* Make list of keys from flattened key structure */
        // start_里面记录下offset
        self.start.push(self.keys.len());
        // 需要多少个key
        // 如果 new_len 大于 len ，则 Vec 由差异扩展，每个额外的插槽都用 value 填充。
        // 如果 new_len 小于 len ，则 Vec 将被截断。
        self.tmp_keys.resize(num_keys, Slice::default());

        // 依次拿到每个key
        for i in 0..num_keys {
            // 拿到key的长度
            let length = self.start[i + 1] - self.start[i];
            // 这里拿到每个key的数据
            let base = &self.keys[self.start[i]..(self.start[i] + length)];

            // 生成相应的key，并且放到tmp_keys里面
            let mut tmp_key = Vec::with_capacity(length);
            tmp_key.write(&base);
            self.tmp_keys[i] = Slice::from_vec(tmp_key);
        }

        // Generate filter for current set of keys and append to result_.
        // 记录下offset
        self.filter_offsets.push(self.result.len() as u32);

        // 利用tmp_keys生成输出，并且放到result里面。
        let mut keys: Vec<&Slice> = Vec::new();
        for tmp_key in &self.tmp_keys {
            keys.push(&tmp_key);
        }
        // let create_filter:Slice = self.policy.create_filter_with_len(num_keys, keys);
        let create_filter: Slice = self.policy.create_filter(keys);
        debug!("create_filter:{:?}.", create_filter);

        self.result.write(create_filter.as_ref());

        // 清空keys/start变量
        self.tmp_keys.clear();
        self.keys.clear();
        self.start.clear();
    }
}

impl FilterBlockReader {
    pub fn new_with_policy(policy: FilterPolicyPtr, contents: &Slice) -> Self {
        let data = Vec::new();
        let offset = Vec::new();

        let contents_len = contents.len();

        // 1 byte for base_lg_ and 4 for start of offset array
        if contents_len < 5 {
            return Self {
                policy,
                data,
                offset,
                num: 0,
                base_lg: 0,
            };
        };

        // let buf = contents.as_ref()[contents_len-5..];

        // let base_lg_ = contentsVe[contents_len-1];

        // let last_word: u32 = Coding::decode_fixed32(buf));
        Self {
            policy,
            data,
            offset,
            num: 0,
            base_lg: 0,
        }
    }

    pub fn key_may_match(&self, block_offset: u64, key: &Slice) -> bool {
        todo!()
    }

    pub fn get_policy(&self) -> FilterPolicyPtr {
        self.policy.clone()
    }

    pub fn get_data(&self) -> Vec<u32> {
        self.data.to_vec()
    }

    pub fn get_offset(&self) -> Vec<u32> {
        self.offset.to_vec()
    }

    pub fn get_num(&self) -> usize {
        self.num
    }

    pub fn get_base_lg(&self) -> usize {
        self.base_lg
    }
}