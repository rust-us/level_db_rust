
pub trait AsHash {
    ///
    ///
    /// # Arguments
    ///
    /// * `data`:
    /// * `n`: data 的长度
    /// * `seed`:  随机数种子
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn hash(data: String, n: usize, seed: u32) -> u32;
}

pub struct Hash {}