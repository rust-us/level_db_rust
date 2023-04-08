use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::slice::Slice;

// #########################  InternalFilterPolicy
pub struct InternalFilterPolicy {
    user_policy_: dyn FilterPolicy
}

impl InternalFilterPolicy {
    fn new(policy: Box<dyn FilterPolicy>) -> Box<InternalFilterPolicy> {
        // InternalFilterPolicy{ user_policy_: policy }
        todo!()
    }
}

impl FilterPolicy for InternalFilterPolicy {
    fn name(&self) -> String {
        todo!()
    }

    fn create_filter(&self, keys: Vec<&Slice>) -> Slice {
        self.create_filter_with_len(keys.len(), keys)
    }

    fn create_filter_with_len(&self, capacity: usize, keys: Vec<&Slice>) -> Slice {
        // 根据指定的参数创建过滤器，并返回结果， 结果为dst的原始内容 + append结果。
        // 参数keys[0,n-1]包含依据用户提供的comparator排序的key列表--可重复，
        // 并把根据这些key创建的filter追加到 dst中。
        //
        todo!()
    }

    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
        todo!()
    }

}