use std::io::Write;
use std::sync::Arc;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::filter_policy_trait::{FilterPolicy, FilterPolicyPtr};
use crate::util::coding::Coding;
use crate::util::slice::Slice;

use crate::util::Result;

// Generate new filter every 2KB of data
const FILTER_BASE_LG: usize = 11;
const FILTER_BASE: usize = 1 << FILTER_BASE_LG;

///
/// meta block 构建器
///
pub trait FilterBlock {
    fn new_with_policy(policy: FilterPolicyPtr) -> Self;

    ///
    /// 构造一个  FilterBlockBuilder
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
    /// * `_block_offset`: 偏移量
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
    policy: FilterPolicyPtr,
    // Flattened key contents
    keys: Vec<u8>,
    // Starting index in keys_ of each key
    start: Vec<usize>,
    // Filter data computed so far
    result: Vec<u8>,
    // policy_->CreateFilter() argument
    tmp_keys: Vec<Slice>,
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
    base_lg: usize
}

impl FilterBlock for FilterBlockBuilder {
    fn new_with_policy(policy: FilterPolicyPtr) -> Self {
        FilterBlock::new_with_policy_capacity(policy, 64)
    }

    fn new_with_policy_capacity(policy: FilterPolicyPtr, capacity: usize) -> Self {
        let keys:Vec<u8> = Vec::with_capacity(capacity);
        let start:Vec<usize> =  Vec::with_capacity(capacity);
        let result:Vec<u8> =  Vec::with_capacity(capacity);
        let tmp_keys:Vec<Slice> = vec![];
        let filter_offsets:Vec<u32> = vec![];

        Self {
            policy,
            keys,
            start,
            result,
            tmp_keys,
            filter_offsets
        }
    }

    fn start_block(&mut self, block_offset: u64) {
        let filter_index = block_offset / (FILTER_BASE as u64);
        assert!(filter_index >= self.filter_offsets.len() as u64);

        while filter_index > self.filter_offsets.len() as u64 {
            self.generate_filter();
        }
    }

    fn add_key_from_str(&mut self, key: &str) {
        self.add_key(&Slice::from(key))
    }

    fn add_key(&mut self, key: &Slice) {
        self.start.push(key.len());
        self.keys.write(key.as_str().as_bytes()).expect("add_key error!");
    }

    fn finish(&mut self) -> Result<Slice> {
        if self.start.len() != 0 {
            self.generate_filter();
        }

        // Append array of per-filter offsets
        let array_offset = self.result.len() as u32;
        // 当前需要写入的位置。result 中可能存在数据，因此为 self.result.len()  的位置
        let mut pos: usize = self.result.len();

        // todo 判断是否需要扩容
        let result_total_capacity = self.result.capacity();

        let dst_append = self.result.as_mut_slice();

        for i in 0..self.filter_offsets.len() {
            // 判断当前 pos + len 4
            let filter_offset_val = self.filter_offsets[i];
            pos = Coding::put_fixed32(dst_append, pos, filter_offset_val);
        }

        pos = Coding::put_fixed32(dst_append, pos, array_offset);

        // Save encoding parameter in result
        // todo 判断是否需要扩容
        Coding::put_varint64(self.result.as_mut_slice(), pos, FILTER_BASE_LG as u64);

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
    fn generate_filter(&mut self) {
        let num_keys = self.start.len();

        if num_keys == 0 {
            // Fast path if there are no keys for this filter
            self.filter_offsets.push(self.result.len() as u32);
            return;
        }

        /* Make list of keys from flattened key structure */
        // Simplify length computation
        self.start.push(self.keys.len());
        // 如果 new_len 大于 len ，则 Vec 由差异扩展，每个额外的插槽都用 value 填充。如果 new_len 小于 len ，则 Vec 将被截断。
        self.tmp_keys.resize(num_keys, Slice::default());

        for i in 0..num_keys {
            let base = &self.keys[self.start[i]..];
            let length = self.start[i+1] - self.start[i];

            let mut tmp_key = Vec::with_capacity(length);
            tmp_key.write(&base);
            self.tmp_keys[i] = Slice::from_vec(tmp_key);
        }

        // Generate filter for current set of keys and append to result_.
        self.filter_offsets.push(self.result.len() as u32);

        let mut keys: Vec<&Slice> = Vec::new();
        keys.push(&self.tmp_keys[0]);
        let create_filter:Slice = self.policy.create_filter_with_len(num_keys, keys);

        // let result_len = self.result.len();
        // let result_total_capacity = self.result.capacity();
        self.result.write(create_filter.as_ref());
        // let result_len = self.result.len();
        // let result_total_capacity = self.result.capacity();

        self.tmp_keys.clear();
        self.keys.clear();
        self.start.clear();
    }
}

impl FilterBlockReader {
    pub fn new_with_policy(policy: FilterPolicyPtr, contents: Slice) -> Self {
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
                base_lg: 0
            }
        };

        // let buf = contents.as_ref()[contents_len-5..];

        // let base_lg_ = contentsVe[contents_len-1];

        // let last_word: u32 = Coding::decode_fixed32(buf));
        Self {
            policy,
            data,
            offset,
            num: 0,
            base_lg: 0
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