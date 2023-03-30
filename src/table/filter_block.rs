use std::sync::Arc;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::slice::Slice;

use crate::util::Result;

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

    fn get_key(&self) -> &str;

    fn get_start(&self) -> Vec<usize>;

    fn get_result(&self) -> &str;

    fn get_tmp_keys(&self) -> Vec<Slice>;

    fn get_tmp_filter_offsets(&self) -> Vec<u32>;
}

/// SSTable 文件里面的 meta block 构建器, 按内存里面指定的格式整理在内存中
pub struct FilterBlockBuilder<FP: FilterPolicy> {
    policy: Arc<FP>,
    // Flattened key contents
    key: String,
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
    data: String,
    // Pointer to beginning of offset array (at block-end)
    offset: String,
    // Number of entries in offset array
    num: usize,
    // Encoding parameter (see kFilterBaseLg in .cc file)
    base_lg: usize
}

impl <FP: FilterPolicy> FilterBlock<FP> for FilterBlockBuilder<FP> {
    fn new_with_policy(policy: Arc<FP>) -> Self {
        let key = String::new();
        let start:Vec<usize> = vec![];
        let result = String::new();
        let tmp_keys:Vec<Slice> = vec![];
        let filter_offsets:Vec<u32> = vec![];

        Self {
            policy,
            key,
            start,
            result,
            tmp_keys,
            filter_offsets
        }
    }

    fn start_block(&mut self, block_offset: u64) {
        self.generate_filter();

        todo!()
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

    fn get_key(&self) -> &str {
        self.key.as_str()
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
        todo!()
    }
}

impl <FP: FilterPolicy> FilterBlockReader<FP> {
    pub fn new_with_policy(policy: Arc<FP>, contents: Slice) -> Self {
        let data = String::new();
        let offset = String::new();

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

    // data,
    // offset,
    // num: 0,
    // base_lg: 0
}