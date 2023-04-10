use std::ops::Mul;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::filter_policy::{AsBloomHash, FromPolicy};
use crate::util::hash::ToHash;
use crate::util::r#const::HASH_DEFAULT_SEED;
use crate::util::slice::Slice;

// #########################  BloomFilterPolicy
pub struct BloomFilterPolicy {
    // 每个key需要多少bit来存储表示
    bits_per_key: usize,

    // k为布隆过滤器重hash function数(hash个数)
    k: usize
}

impl BloomFilterPolicy {
    ///
    ///
    /// Return a new filter policy that uses a bloom filter with approximately the specified number of bits per key.
    /// A good value for bits_per_key is 10, which yields a filter with ~ 1% false positive rate.
    ///
    /// # Arguments
    ///
    /// * `bits_per_key`:   m位的bit数组 / n个整数set 的值
    ///
    /// returns: BloomFilterPolicy
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new() -> Self {
        BloomFilterPolicy::new_with_bits_per_key(10)
    }

    pub fn new_with_bits_per_key(bits_per_key: usize) -> Self {
        // We intentionally round down to reduce probing cost a little bit
        // 最优的 k_ 是 ln2 * (m/n) -> factor * bits_per_key

        // factor = 0.69 =~ ln(2)
        let factor: f64 = 0.69;
        let mut k_: usize = factor.mul(bits_per_key as f64).round() as usize;

        // 计算哈希函数个数，控制在 1~30个范围。
        if k_ < 1 {
            k_ = 1;
        }
        if k_ > 30{
            k_ = 30;
        }

        Self {
            bits_per_key,
            k : k_
        }
    }
}

impl<'a> BloomFilterPolicy {
    pub fn bloom_hash(key: &Slice) -> u32 {
        key.to_hash_with_seed(HASH_DEFAULT_SEED)
    }
}

/// get struct  BloomFilterPolicy 属性
impl FromPolicy for BloomFilterPolicy {
    fn from_bits_per_key(&self) -> usize {
        self.bits_per_key
    }

    fn from_k(&self) -> usize {
        self.k
    }
}

impl FilterPolicy for BloomFilterPolicy {

    fn name(&self) -> String {
        String::from("leveldb.BuiltinBloomFilter")
    }

    fn create_filter(&self, keys: Vec<&Slice>) -> Slice {
        let len: usize = keys.len();

        self.create_filter_with_len(len, keys)
    }

    fn create_filter_with_len(&self, capacity: usize, keys: Vec<&Slice>) -> Slice {
        let n: usize = capacity;

        // Compute bloom filter size (in both bits and bytes)
        // 计算出中的需要的bits个数, n * bits_per_key, 也就是说，对于每一个key需要这么多bit
        // 因为bits_per_key_表示 m／n，所以bits = bits_per_key_ * n = m(m 的意思是： m位的bit数组)
        let mut bits: usize = n * self.bits_per_key;

        // For small n, we can see a very high false positive rate. Fix it by enforcing a minimum bloom filter length.
        // bits太小的话会导致很高的查询错误率， 这里强制bits个数不能小于64
        if bits < 64 {
            bits = 64;
        }

        //向上按8bit，一个Byte对齐
        let bytes: usize = (bits + 7) / 8;
        // 根据 bytes 算出bits数
        bits = bytes * 8;

        // 扩展下要存储BloomFilter的内存空间， 并在尾部一个Byte存哈希函数的个数。
        let mut dst_chars: Vec<u8> = vec![0; bytes + 1]; // 相当于是 append 了bytes个0
        // 在filter的最后压入哈希函数的个数。 在最后一位， 记录k 值。 这个k是位于bytes之后。
        dst_chars[bytes] = self.k as u8;

        // 开始依次存储每个key值。
        // 对于每个key采用double hash的方式生成k_个bitpos，然后在 dst_chars 的相应位置设置1。
        for i in 0..keys.len() {
            let slice = keys[i];

            /* 计算哈希值 */
            // BloomFilter理论是通过多个hash计算来减少冲突，
            // 但leveldb实际上并未真正去计算多个hash，而是通过double-hashing的方式来达到同样的效果。
            // double-hashing的理论如下：
            //      h(i,k) = (h1(k) + i*h2(k)) % T.size
            //      h1(k) = h, h2(k) = delta, h(i,k) = bitpos
            //
            // 1、计算hash值；
            // 2、hash值的高15位，低17位对调
            // 3、按k_个数来存储当前hash值。
            //      3-1、计算存储位置；
            //      3-2、按bit存；
            //      3-3、累加hash值用于下次计算
            //
            // Use double-hashing to generate a sequence of hash values.
            // See analysis in [Kirsch,Mitzenmacher 2006].
            let mut h : u32 = slice.bloom_hash();
            // Rotate right 17 bits
            let delta : u32 = (h >> 17) | (h << 15);

            for j in 0..self.k {
                let bitpos:usize = ((h as usize) % bits);

                // val ==> 1 << (bitpos % 8)
                let mod_val: usize = bitpos % 8;
                let val = (1 as u8).wrapping_shl(mod_val as u32);

                // 本来应该直接把h bit设置为1的。但是这里总共只有bits个bit, 访问m[i] 把相应位设置为1
                // a |= b  ==>  按位或， 后赋值给a
                // let position: usize = bitpos / 8;
                dst_chars[bitpos / 8] |= val;

                // 累加来实现k个hash函数, h.wrapping_add(delta) ==> h += delta
                // LevelDB中并没有真正创建k个哈希函数。而是使用旧有的哈希值累加。
                // 使用了最原始的h哈希值位移来得到。(h >> 17) | (h << 15);，累加delta得到下一次hash值。
                h = h.wrapping_add(delta);
            }
        }

        // Vec<u8> 转 Slice
        Slice::from_buf(&dst_chars)
    }

    // fn create_filter_u8(&self, keys: Vec<u8>) -> Slice {
    //     self.create_filter_u8_with_len(keys.len(), keys)
    // }
    //
    // fn create_filter_u8_with_len(&self, capacity: usize, keys: Vec<u8>) -> Slice {
    //     todo!()
    // }

    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
        // 1、插入时按1Byte对齐；
        // 2、尾部插入了一个Byte的hash个数
        // 所以大小不能小于2个字节
        let len: usize = bloom_filter.size();
        if len < 2 {
            return false;
        }

        // 获得相应的内存区域的数据: 除去尾部的1Byte对应的hash个数，就是当前位数组容器的大小
        let bloom_filter_array:Vec<u8>  = bloom_filter.to_vec();
        // 总共的bits数目
        let bits: usize = (len - 1) * 8;

        // 取得k哈希函数的数目
        // Use the encoded k so that we can read filters generated by bloom filters created using different parameters.
        let k: u8 = bloom_filter_array[len - 1];
        // 对于大于30个哈希函数的情况，这里直接返回存在
        if k > 30 {
            // Reserved for potentially new encodings for short bloom filters.  Consider it a match.
            return true;
        }

        // 1、计算查询key对应的hash值
        // 2、按插入规则去 &，只要有1bit不相同，那就不存在。

        // 计算哈希值
        let mut h : u32 = key.bloom_hash();
        // Rotate right 17 bits
        let delta = (h >> 17) | (h << 15);

        // 计算key的hash值，重复计算阶段的步骤，循环计算k_个hash值，只要有一个结果对应的bit位为0，就认为不匹配，否则认为匹配
        for j in 0..k {
            let bitpos:usize = ((h as usize) % bits);
            if (bloom_filter_array[bitpos/8] & (1 << (bitpos % 8))) == 0 {
                return false;
            }

            h = h.wrapping_add(delta);
        }

        return true;
    }
}