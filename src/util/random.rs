pub struct Random {
    seed: u32,
}

impl Random {
    /// 根据种子构造随机数
    ///
    /// # Arguments
    ///
    /// * `s`: seed 种子
    ///
    /// returns: Random
    ///
    /// # Examples
    ///
    /// ```
    /// let random = Random::new(8192);
    /// ```
    pub fn new(s: u32) -> Random {
        // Avoid bad seeds.
        Random {
            seed: match s {
                0 => 1,
                2147483647 => 1,
                _ => s
            }
        }
    }
    /// 获取一个随机数
    ///
    /// # Examples
    ///
    /// ```
    /// let num = random.next();
    /// ```
    pub fn next(&mut self) -> u32 {
        // mod
        let m = 2147483647_u32;
        let a = 16807_u32;
        let product = self.seed * a;
        self.seed = (product >> 31) + (product & m);
        if self.seed > m { self.seed -= m }
        self.seed
    }
    /// 生成随机数并对n取模
    ///
    /// # Arguments
    ///
    /// * `n`: 模
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    /// let num = random.uniform(10);
    /// ```
    pub fn uniform(&mut self, n: u32) -> u32 {
        self.next() % n
    }

    /// 生成随机数对n取模并判断是否为0
    ///
    /// # Arguments
    ///
    /// * `n`: 模
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// let is_zero = random.one_in(2);
    /// ```
    pub fn one_in(&mut self, n: u32) -> bool {
        (self.next() % n) == 0
    }
    /// 对随机数进行倾斜
    ///
    /// # Arguments
    ///
    /// * `max_log`: 最大范围
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    /// let skewed_num = random.skewed(10);
    /// ```
    pub fn skewed(&mut self, max_log: u32) -> u32 {
        let bits = 1 << self.uniform(max_log + 1);
        self.uniform(bits)
    }
}