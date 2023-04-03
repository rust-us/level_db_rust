use std::error::Error;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::sync::Arc;
use crate::db::file_meta_data::FileMetaData;
use crate::db::filename::FileName;
use crate::db::table_cache::TableCache;
use crate::table::table::Table;
use crate::table::table_builder::TableBuilder;
use crate::traits::DataIterator;
use crate::util::env::Env;
use crate::util::options::{Options, ReadOptions};
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
        // 迭代器移动到第一个节点
        iter.seek_to_first();
        // 生成一个 SSTable 文件名
        let file_name = FileName::table_file_name(dbname, meta.get_number());

        let mut s : Status = Status::default();

        if iter.valid() {
            let fileRS: Result<File> = env.new_writable_file(&file_name);
            if(!fileRS.is_ok()){
                return Err(fileRS.err().unwrap());
            }

            let writable_file = Arc::new(fileRS.unwrap());
            // 生成一个 TableBuilder
            let builder: TableBuilder = TableBuilder::new_with_writable_file(options, writable_file.clone());

            meta.get_smallest().decode_from(&iter.key());

            // 调用迭代器，依次将每个键-值对加入 TableBuilder
            while iter.valid() {
                iter.next();

                let key = iter.key();
                meta.get_largest().decode_from(&key);
                builder.add(&key, &iter.value());
            }

            // Finish and check for builder errors
            // 调用 TableBuilder 的 Finish 函数生成 SSTable 文件
            s = builder.finish();
            if s.is_ok() {
                meta.set_file_size(builder.get_file_size());
                assert!(meta.get_file_size() > 0);
            }

            // Finish and check for file errors
            // 将文件刷新到磁盘
            if s.is_ok() {
                let rs:io::Result<()> = writable_file.sync_data();
                if rs.is_ok() {
                    s = Status::default();
                }else{
                    s = Status::wrapper_str(LevelError::KIOError, rs.unwrap_err().to_string().as_str());
                }
            }
            // 关闭文件
            // if s.is_ok() {
            //     writableFile.close
            // }

            if s.is_ok() {
                let readOptions = ReadOptions::default();
                // Verify that the table is usable
                let it: Box<dyn DataIterator> = table_cache.new_iterator(&readOptions,
                                         meta.get_number(),
                                         meta.get_file_size() as usize,
                                         &Table::new())
                    .expect("table_cache.new_iterator error");
                s = it.status();
            }
        } // if end

        // Check for input iterator errors
        if !iter.status().is_ok() {
            s = iter.status();
        }

        if s.is_ok() && meta.get_file_size() > 0 {
            // Keep it
        } else {
            // DeleteFile fname
            // todo
        }

        if s.is_ok() {
            // todo
            // return Ok(meta);
            return Ok(FileMetaData::default());
        }else{
            return Err(s);
        }
    }
}