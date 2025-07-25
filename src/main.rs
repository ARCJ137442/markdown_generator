use ignore::Walk;
use phf::phf_map;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fmt::Write;
use std::fs;
use std::io;
use std::path::Path;

/// 获取文件扩展名
fn get_file_extension(path: &Path) -> &str {
    path.extension().and_then(OsStr::to_str).unwrap_or_default()
}

/// 代码块语言映射表
static CODEBLOCK_LANG_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    // 配置/纯文本
    "md" => "markdown",
    "json" => "json",
    "yml" => "yaml",
    "toml" => "toml",
    "ini" => "ini",
    "txt" => "plaintext",
    "yaml" => "yaml",
    "xml" => "xml",
    "xslt" => "xml",
    "xsl" => "xml",
    "xsd" => "xml",
    // Python
    "py" => "python",
    "pyi" => "python",
    // C/C++
    "c" => "c",
    "h" => "cpp",
    "cpp" => "cpp",
    "hpp" => "cpp",
    "inl" => "cpp",
    // Java
    "java" => "java",
    // Rust
    "rs" => "rust",
    // Kotlin
    "kt" => "kotlin",
    // Julia
    "jl" => "julia",
    // Bash/Shell
    "sh" => "bash",
    "bash" => "bash",
    "cmd" => "cmd",
    "bat" => "cmd",
    "ps1" => "powershell",
    // Lua
    "lua" => "lua",
    // C#
    "cs" => "csharp",
    // JavaScript/TypeScript
    "ts" => "typescript",
    "tsx" => "typescript",
    "js" => "javascript",
    "jsx" => "javascript",
    // Go
    "go" => "go",
    // Zig
    "zig" => "zig",
    // Nim
    "nim" => "nim",
    // BEAM: Elixir/Erlang/Gleam
    "gleam" => "gleam",
    "exs" => "elixir",
    "ex" => "elixir",
    "erl" => "erlang",
    "hrl" => "erlang",
    // Haskell
    "hs" => "haskell",
    "lhs" => "haskell",
    // OCaml
    "ml" => "ocaml",
    "mli" => "ocaml",
    // Perl
    "pl" => "perl",
    "pm" => "perl",
    // PHP
    "php" => "php",
    // Ruby
    "rb" => "ruby",
    "erb" => "ruby",
    // R
    "r" => "r",
    // reStructuredText
    "rst" => "rst",
    // SQL
    "sql" => "sql",
    // Swift
    "swift" => "swift",
    // Vue
    "vue" => "vue",
    // CSS
    "css" => "css",
    "scss" => "scss",
    "less" => "less",
};

fn codeblock_lang(extension: &str) -> &str {
    CODEBLOCK_LANG_MAP.get(extension).cloned().unwrap_or("")
}

macro_rules! push_str {
    ($target:expr, $($arg:tt)+) => {
        write!($target, $($arg)+).expect(concat!("Can't write to", stringify!($target)))
    };
}

fn push_title(target: &mut String, title: impl Display, title_level: usize) {
    // 标题
    push_str!(target, "{} `{title}`\n\n", "#".repeat(title_level));
}

/// 将文件内容转换为markdown标题和代码块
fn file_to_markdown(root_path: &Path, file_path: &Path, title_level: usize) -> io::Result<String> {
    let content = fs::read_to_string(file_path)?;
    let extension = get_file_extension(file_path);
    let lang = match codeblock_lang(&extension) {
        "" => extension,
        lang => lang,
    };

    let mut result = String::new();

    // 添加标题
    push_title(
        &mut result,
        file_path
            .strip_prefix(root_path)
            .map_err(|e| eprintln!("Can't strip prefix: {e}"))
            .unwrap_or(file_path)
            .display(),
        title_level,
    );

    // 添加代码块
    push_str!(result, "````{lang}\n");
    push_str!(result, "{content}");
    push_str!(result, "````\n\n");

    Ok(result)
}

/// 遍历目录，生成markdown文件
fn comprehend_path_to_markdown(
    root_path: &Path,
    file_filter: impl Fn(&Path) -> bool,
) -> io::Result<String> {
    let mut result = String::new();

    let mut root_path = root_path.to_path_buf();
    println!("root = {}", root_path.display());

    let mut n_files = 0_usize;
    let mut n_folders = 0_usize;

    for entry_r in Walk::new(&root_path) {
        let entry = entry_r.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let path = entry.path();
        let root_title = path.file_name().and_then(OsStr::to_str).unwrap_or("");

        if entry.depth() == 0 {
            root_path = path.to_path_buf();
        }

        // 根目录=0，根目录下文件(夹)=1
        let level = entry.depth() + 1;
        if path.is_dir() {
            println!("dir {}", path.display());
            n_folders += 1;
            push_title(
                &mut result,
                match level {
                    1 => Path::new(root_title),
                    _ => path
                        .strip_prefix(&root_path)
                        .map_err(|e| eprintln!("Can't strip prefix: {e}"))
                        .unwrap_or(path),
                }
                .display(),
                level,
            );
        }
        // Process files in this directory
        else if path.is_file() && file_filter(path) {
            println!("convert {}", path.display());
            n_files += 1;
            match file_to_markdown(&root_path, &path, level) {
                Ok(md) => push_str!(result, "{md}"),
                // 捕获UTF-8错误：一般是二进制文件，不应展示
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    println!("Ignored binary file: `{}`", path.display())
                }
                Err(e) => eprintln!("Can't read file `{}`: {e:?}", path.display()),
            }
        }
    }

    println!("Done with {} bytes, {n_files} files, {n_folders} folders.", result.len());
    Ok(result)
}

fn main() -> io::Result<()> {
    // 读取命令行参数，若包含「--all」遍历所有文件，否则只遍历「代码块表」中有的
    let args: Vec<String> = std::env::args().collect();
    let arg_all = args.contains(&"--all".to_string());
    let file_filter: Box<dyn Fn(&Path) -> bool> = if arg_all {
        Box::new(|_| true)
    } else {
        Box::new(|path| CODEBLOCK_LANG_MAP.contains_key(&get_file_extension(path)))
    };
    let current_dir = std::env::current_dir().expect("Can't get current directory");
    let root_path = Path::new(&current_dir);
    let markdown_content = comprehend_path_to_markdown(root_path, file_filter)?;

    let mut file = fs::File::create("output.md")?;
    use std::io::Write;
    file.write_all(markdown_content.as_bytes())?;

    Ok(())
}
