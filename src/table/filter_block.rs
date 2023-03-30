use std::io::Write;
use std::sync::Arc;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::coding::Coding;
use crate::util::slice::Slice;

use crate::util::Result;

// Generate new filter every 2KB of data
const FILTER_BASE_LG: usize = 11;
const FILTER_BASE: usize = 1 << FILTER_BASE_LG;

///
/// meta block 构建器
///
pub trait FilterBlock<FP: FilterPolicy> {

    ///
    /// 构造一个  FilterBlockBuilder
    ///
    /// # Arguments
    ///
    /// * `policy`:
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use level_db_rust::util::filter_policy::BloomFilterPolicy;
    ///
    /// let policy = Arc::new(BloomFilterPolicy::new(2));
    /// let filter_block: FilterBlockBuilder<BloomFilterPolicy> = FilterBlockBuilder::new_with_policy(policy);
    /// ```
    #[inline]
    fn new_with_policy(policy: Arc<FP>) -> Self;

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
    #[inline]
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

    fn get_policy(&self) -> Box<&FP>;

    fn get_keys(&self) -> &str;

    fn get_start(&self) -> Vec<usize>;

    fn get_result(&self) -> &str;

    fn get_tmp_keys(&self) -> Vec<Slice>;

    fn get_tmp_filter_offsets(&self) -> Vec<u32>;
}

/// SSTable 文件里面的 meta block 构建器, 按内存里面指定的格式整理在内存中
pub struct FilterBlockBuilder<FP: FilterPolicy> {
    policy: Arc<FP>,
    // Flattened key contents
    keys: String,
    // Starting index in keys_ of each key
    start: Vec<usize>,
    // Filter data computed so far
    result: String,
    // policy_->CreateFilter() argument
    tmp_keys: Vec<Slice>,
    filter_offsets: Vec<u32>,
}

pub struct FilterBlockReader<FP: FilterPolicy> {
    policy: Arc<FP>,
    // Pointer to filter data (at block-start)
    data: Vec<u32>,
    // Pointer to beginning of offset array (at block-end)
    offset: Vec<u32>,
    // Number of entries in offset array
    num: usize,
    // Encoding parameter (see kFilterBaseLg in .cc file)
    base_lg: usize
}

impl <FP: FilterPolicy> FilterBlock<FP> for FilterBlockBuilder<FP> {
    fn new_with_policy(policy: Arc<FP>) -> Self {
        let keys = String::new();
        let start:Vec<usize> = vec![];
        let result = String::new();
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
        todo!()
    }

    fn finish(&mut self) -> Result<Slice> {
        self.generate_filter();

        todo!()
    }

    fn get_policy(&self) -> Box<&FP> {
        Box::new(self.policy.as_ref())
    }

    fn get_keys(&self) -> &str {
        self.keys.as_str()
    }

    fn get_start(&self) -> Vec<usize> {
        self.start.to_vec()
    }

    fn get_result(&self) -> &str {
        self.result.as_str()
    }

    fn get_tmp_keys(&self) -> Vec<Slice> {
        self.tmp_keys.to_vec()
    }

    fn get_tmp_filter_offsets(&self) -> Vec<u32> {
        self.filter_offsets.to_vec()
    }
}

impl <FP: FilterPolicy> FilterBlockBuilder<FP> {
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
            let base = &self.keys.as_bytes()[self.start[i]..];
            let length = self.start[i+1] - self.start[i];

            let mut tmp_key = Vec::with_capacity(length);
            tmp_key.write(&base);
            self.tmp_keys[i] = Slice::from_vec(tmp_key);
        }

        // Generate filter for current set of keys and append to result_.

    }
}

impl <FP: FilterPolicy> FilterBlockReader<FP> {
    pub fn new_with_policy(policy: Arc<FP>, contents: Slice) -> Self {
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

    pub fn get_policy(&self) -> Box<&FP> {
        Box::new(self.policy.as_ref())
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