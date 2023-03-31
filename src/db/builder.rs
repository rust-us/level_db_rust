use std::ops::Deref;
use std::sync::Arc;
use crate::db::file_meta_data::FileMetaData;
use crate::db::filename::FileName;
use crate::db::table_cache::TableCache;
use crate::table::table_builder::TableBuilder;
use crate::traits::DataIterator;
use crate::util::env::{Env, WritableFile};
use crate::util::options::{Options};
use crate::util::Result;
use crate::util::slice::Slice;
use crate::util::status::{LevelError, Status};

pub struct BuildTable {}

impl BuildTable {

    ///
    /// 生成 SSTable
    ///
    /// Build a Table file from the contents of *iter.
    /// The generated file will be named according to meta->number.
    /// On success, the rest of meta will be filled with metadata about the generated table.
    /// If no data is present in *iter, meta->file_size will be set to zero, and no Table file will be produced.
    ///
    /// # Arguments
    ///
    /// * `dbname`:
    /// * `env`:
    /// * `options`:
    /// * `table_cache`:
    /// * `iter`:
    /// * `meta`:
    ///
    /// returns: Result<FileMetaData, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn build_table(dbname: &Slice, env: &Env, options: &Options,
                       table_cache: &TableCache, mut iter: Box<dyn DataIterator>,
                       meta: &mut FileMetaData) -> Result<FileMetaData> {
        meta.set_file_size(0);
        iter.seek_to_first();

        let file_name = FileName::table_file_name(dbname, meta.get_number());

        if iter.valid() {
            let fileRS: Result<WritableFile> = env.new_writable_file(&file_name);
            if(!fileRS.is_ok()){
                return Err(fileRS.err().unwrap());
            }

            let writableFile = Arc::new(fileRS.unwrap());
            let builder: TableBuilder = TableBuilder::new_with_writable_file(options, writableFile);

            meta.get_smallest().decode_from(&iter.key());
            while iter.valid() && iter.has_next(){
                iter.next();

                let key = iter.key();
                meta.get_largest().decode_from(&key);
                // builder.add(key, iter.value());
            }
        }


        Err(Status::wrapper_str(LevelError::KBadRecord, "a"))
    }
}