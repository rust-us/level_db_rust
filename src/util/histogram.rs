use std::fmt::{Display, Formatter};
/// 直方图的桶个数
const K_NUM_BUCKETS: usize = 154;
/// 直方图每个桶可存放值的上界，开区间
const K_BUCKET_LIMIT: [f64; K_NUM_BUCKETS] = [
    1.0,
    2.0,
    3.0,
    4.0,
    5.0,
    6.0,
    7.0,
    8.0,
    9.0,
    10.0,
    12.0,
    14.0,
    16.0,
    18.0,
    20.0,
    25.0,
    30.0,
    35.0,
    40.0,
    45.0,
    50.0,
    60.0,
    70.0,
    80.0,
    90.0,
    100.0,
    120.0,
    140.0,
    160.0,
    180.0,
    200.0,
    250.0,
    300.0,
    350.0,
    400.0,
    450.0,
    500.0,
    600.0,
    700.0,
    800.0,
    900.0,
    1000.0,
    1200.0,
    1400.0,
    1600.0,
    1800.0,
    2000.0,
    2500.0,
    3000.0,
    3500.0,
    4000.0,
    4500.0,
    5000.0,
    6000.0,
    7000.0,
    8000.0,
    9000.0,
    10000.0,
    12000.0,
    14000.0,
    16000.0,
    18000.0,
    20000.0,
    25000.0,
    30000.0,
    35000.0,
    40000.0,
    45000.0,
    50000.0,
    60000.0,
    70000.0,
    80000.0,
    90000.0,
    100000.0,
    120000.0,
    140000.0,
    160000.0,
    180000.0,
    200000.0,
    250000.0,
    300000.0,
    350000.0,
    400000.0,
    450000.0,
    500000.0,
    600000.0,
    700000.0,
    800000.0,
    900000.0,
    1000000.0,
    1200000.0,
    1400000.0,
    1600000.0,
    1800000.0,
    2000000.0,
    2500000.0,
    3000000.0,
    3500000.0,
    4000000.0,
    4500000.0,
    5000000.0,
    6000000.0,
    7000000.0,
    8000000.0,
    9000000.0,
    10000000.0,
    12000000.0,
    14000000.0,
    16000000.0,
    18000000.0,
    20000000.0,
    25000000.0,
    30000000.0,
    35000000.0,
    40000000.0,
    45000000.0,
    50000000.0,
    60000000.0,
    70000000.0,
    80000000.0,
    90000000.0,
    100000000.0,
    120000000.0,
    140000000.0,
    160000000.0,
    180000000.0,
    200000000.0,
    250000000.0,
    300000000.0,
    350000000.0,
    400000000.0,
    450000000.0,
    500000000.0,
    600000000.0,
    700000000.0,
    800000000.0,
    900000000.0,
    1000000000.0,
    1200000000.0,
    1400000000.0,
    1600000000.0,
    1800000000.0,
    2000000000.0,
    2500000000.0,
    3000000000.0,
    3500000000.0,
    4000000000.0,
    4500000000.0,
    5000000000.0,
    6000000000.0,
    7000000000.0,
    8000000000.0,
    9000000000.0,
    1e200_f64];

/// 直方图
pub struct Histogram {
    min: f64,
    max: f64,
    num: f64,
    sum: f64,
    sumSquares: f64,
    buckets: [f64; K_NUM_BUCKETS]
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            min: K_BUCKET_LIMIT[K_NUM_BUCKETS - 1],
            max: 0.0,
            num: 0.0,
            sum: 0.0,
            sumSquares: 0.0,
            buckets: [0.0; K_NUM_BUCKETS]
        }
    }
}

/// 实现Display，自动提供to_string
impl Display for Histogram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut print = String::new();
        print.push_str(&format!("Count: {:.0} Average: {:.4} StdDev: {:.2} \n",
                                self.num, self.average(), self.standard_deviation()));
        print.push_str(&format!("Min: {:.4} Median: {:.4} Max: {:.4} \n",
                                self.min, self.median(), self.max));
        print.push_str("------------------------------------------------------\n");

        let mult = 100.0 / self.num;
        let mut sum:f64 = 0.0;
        for i in 0..K_NUM_BUCKETS {
            if self.buckets[i] <= 0.0 {
                continue;
            }
            sum += self.buckets[i];
            print.push_str(&format!("[ {:>7.0}, {:>7.0} ) {:>7.0} {:>7.3}% {:>7.3}% ",
                                    if i == 0 {0.0} else {K_BUCKET_LIMIT[i - 1]}, // 左端点
                                    K_BUCKET_LIMIT[i], // 右端点
                                    self.buckets[i], // 计数
                                    mult * self.buckets[i], // 百分比
                                    mult * sum)); // 累计百分比
            // 基于百分比的hash marks；100%为20 mark
            let mark:f64 = 20.0 * (self.buckets[i] / self.num);
            print.push_str(&format!("{:.0}#\n", mark));
        }
        write!(f, "{}", print)
    }
}

impl Histogram {
    /// 清空直方图数据
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut histogram = Histogram::default();
    /// histogram.clear();
    /// ```
    pub fn clear(&mut self) {
        self.min = K_BUCKET_LIMIT[K_NUM_BUCKETS - 1];
        self.max = 0.0;
        self.num = 0.0;
        self.sum = 0.0;
        self.sumSquares = 0.0;
        for i in 0..K_NUM_BUCKETS {
            self.buckets[i] = 0.0;
        }
    }
    /// 向直方图中添加一条新值
    ///
    /// # Arguments
    ///
    /// * `value`: 待添加的值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut histogram = Histogram::default();
    /// histogram.add(1.0);
    /// ```
    pub fn add(&mut self, value: f64) {
        let mut index = 0;
        // 找到值所属的桶
        while index < K_NUM_BUCKETS - 1 && K_BUCKET_LIMIT[index] <= value {
            index += 1;
        }
        self.buckets[index] += 1.0;
        if self.min > value {
            self.min = value;
        }
        if self.max < value {
            self.max = value;
        }
        self.num += 1.0;
        self.sum += value;
        self.sumSquares += value * value;
    }
    /// 合并另一个直方图的数据到当前直方图
    ///
    /// # Arguments
    ///
    /// * `other`:另一个直方图
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut histogram = Histogram::default();
    /// let other = Histogram::default();
    /// histogram.merge(other);
    /// ```
    pub fn merge(&mut self, other: &Self) {
        if self.min > other.min {
            self.min = other.min;
        }
        if self.max < other.max {
            self.max = other.max;
        }
        self.num += other.num;
        self.sum += other.sum;
        self.sumSquares += other.sumSquares;
        for i in 0..K_NUM_BUCKETS {
            self.buckets[i] += other.buckets[i];
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `p`:
    ///
    /// returns: f64
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn percentile(&self, p :f64) -> f64 {
        let threhold = self.num * ( p / 100.0);
        let mut sum :f64 = 0.0;
        for i in 0..K_NUM_BUCKETS {
            sum += self.buckets[i];
            if sum >= threhold {
                let leftPoint :f64 = if i == 0 {0.0} else {K_BUCKET_LIMIT[i - 1]};
                let rightPoint :f64 = K_BUCKET_LIMIT[i];
                let leftSum :f64 = sum - self.buckets[i];
                let rightSum :f64 = sum;
                let pos :f64 = (threhold - leftSum) / (rightSum - leftSum);
                let mut r :f64 = leftPoint + (rightPoint - leftPoint) * pos;
                if r < self.min {
                    r = self.min;
                }
                if r > self.max {
                    r = self.max;
                }
                return r;
            }
        }
        return self.max;
    }

    fn median(&self) -> f64 {
       return self.percentile(50.0);
    }

    /// 求平均值
    fn average(&self) -> f64 {
        if self.sum == 0.0 {
            return 0.0;
        }
        return self.sum / self.num;
    }

    fn standard_deviation(&self) -> f64 {
        if self.sum == 0.0 {
            return 0.0;
        }
        let variance = (self.sumSquares * self.num - self.sum * self.sum) / (self.num * self.num);
        return variance.sqrt();
    }
}