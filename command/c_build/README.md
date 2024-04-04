## 添加C语言方法

确保你的系统中存在C/C++环境与gcc工具链

- 向文件夹c中加入你的c代码后
-  chmod +x ./build_c.sh
-  Usage: ./build_c.sh <source_file> <output_object_file(.c)> <output_shared_library_file(.so)>


完成后，可在command.rs中声明方法，并在arg.rs中进行配置

```bash
    ./build_c.sh vim vim.c vim.so
```


```rust
    extern {
        fn vim_edit(filename: *const libc::c_char);
    }

    #[link(name = "vim")]
    extern {}

    fn main() {
        let filename = "example.txt";
        unsafe {
            let filename_c = std::ffi::CString::new(filename).expect("CString::new failed");
            vim_edit(filename_c.as_ptr());
        }
    }
```