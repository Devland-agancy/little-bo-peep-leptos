import argv
import gleam/io
import gleam/iterator
import gleam/list
import gleam/result
import gleam/string
import simplifile

const content_dir = "../src/content/"

fn is_commented(parent_path, name) {
  let dir_children =
    result.unwrap(simplifile.read_directory(content_dir <> parent_path), [])

  let is_commented = list.any(dir_children, fn(child) { child == "#" <> name })

  is_commented
}

fn rename(from, to) {
  let _ = case string.ends_with(from, "__parent_emu.rs") {
    True -> {
      io.println("Skipping __parent_emu.rs")
    }
    False -> {
      let _ = case result.unwrap(simplifile.is_file(from), True) {
        True -> {
          simplifile.rename_file(from, to)
        }
        False -> {
          simplifile.rename_directory(from, to)
        }
      }
      Nil
    }
  }
}

fn comment(path, comment_out) {
  // comment_out means we add # at start of file if not already added
  let splits = string.split(path, "/")
  let splits_poped = list.take(splits, list.length(splits) - 1)
  let parent_path = string.join(splits_poped, "/")
  let name = result.unwrap(list.last(splits), "")

  let _ = case is_commented(parent_path, name), comment_out {
    True, True -> {
      io.println("Already commented")
    }
    True, False -> {
      let _ =
        rename(
          content_dir <> parent_path <> "/#" <> name,
          content_dir <> parent_path <> "/" <> name,
        )

      io.println("Uncommented !!")
    }
    False, True -> {
      let _ =
        rename(
          content_dir <> parent_path <> "/" <> name,
          content_dir <> parent_path <> "/#" <> name,
        )

      io.println("Commented !!")
    }
    False, False -> {
      io.println("Already not commented")
    }
  }
}

fn un_comment_children(parent_path, real_name) {
  let _ = case
    simplifile.is_directory(content_dir <> parent_path <> "/" <> real_name)
  {
    Ok(is_dir) -> {
      let _ = case is_dir, string.starts_with(real_name, "#") {
        True, True -> {
          let renamed = string.drop_left(real_name, 1)
          let _ =
            rename(
              content_dir <> parent_path <> "/" <> real_name,
              content_dir <> parent_path <> "/" <> renamed,
            )
          let children =
            result.unwrap(
              simplifile.read_directory(
                content_dir <> parent_path <> "/" <> renamed,
              ),
              [],
            )
          let _ =
            list.each(children, fn(child) {
              un_comment_children(parent_path <> "/" <> renamed, child)
            })
        }
        True, False -> {
          let children =
            result.unwrap(
              simplifile.read_directory(
                content_dir <> parent_path <> "/" <> real_name,
              ),
              [],
            )
          let _ =
            list.each(children, fn(child) {
              un_comment_children(parent_path <> "/" <> real_name, child)
            })
        }
        False, True -> {
          let _ =
            rename(
              content_dir <> parent_path <> "/" <> real_name,
              content_dir
                <> parent_path
                <> "/"
                <> string.drop_left(real_name, 1),
            )
          Nil
        }
        False, False -> {
          Nil
        }
      }
    }
    Error(_) -> {
      io.println("Could not open file")
    }
  }
}

fn comment_children(parent_path, real_name) {
  let _ = case
    simplifile.is_directory(content_dir <> parent_path <> "/" <> real_name)
  {
    Ok(is_dir) -> {
      let _ = case is_dir, string.starts_with(real_name, "#") {
        True, True -> {
          let children =
            result.unwrap(
              simplifile.read_directory(
                content_dir <> parent_path <> "/" <> real_name,
              ),
              [],
            )
          let _ =
            list.each(children, fn(child) {
              comment_children(parent_path <> "/" <> real_name, child)
            })
        }
        True, False -> {
          let renamed = case string.is_empty(real_name) {
            False -> {
              let _ =
                rename(
                  content_dir <> parent_path <> "/" <> real_name,
                  content_dir <> parent_path <> "/" <> "#" <> real_name,
                )
              "#" <> real_name
            }
            True -> {
              real_name
              // which is ""
            }
          }
          let children =
            result.unwrap(
              simplifile.read_directory(
                content_dir <> parent_path <> "/" <> renamed,
              ),
              [],
            )
          let _ =
            list.each(children, fn(child) {
              comment_children(parent_path <> "/" <> renamed, child)
            })
        }
        False, True -> {
          Nil
        }
        False, False -> {
          let _ =
            rename(
              content_dir <> parent_path <> "/" <> real_name,
              content_dir <> parent_path <> "/#" <> real_name,
            )
          Nil
        }
      }
    }
    Error(_) -> {
      io.println("Could not open file")
    }
  }
}

fn comment_ascendors_siblings(path) {
  case path {
    "" -> {
      Nil
    }
    _ -> {
      let splits = string.split(path, "/")
      let splits_poped = list.take(splits, list.length(splits) - 1)
      let parent_path = string.join(splits_poped, "/")
      let name = result.unwrap(list.last(splits), "")
      // Uncomment ascendors
      let renamed = case string.starts_with(name, "#") {
        True -> {
          rename(
            content_dir <> parent_path <> "/" <> name,
            content_dir <> parent_path <> "/" <> string.drop_left(name, 1),
          )
          string.drop_left(name, 1)
        }
        False -> {
          name
        }
      }

      //comment the siblings
      let dir_children =
        result.unwrap(simplifile.read_directory(content_dir <> parent_path), [])

      let _ =
        list.each(dir_children, fn(child) {
          let _ = case string.starts_with(child, "#") || child == renamed {
            False -> {
              let renamed = "#" <> child
              let _ =
                rename(
                  content_dir <> parent_path <> "/" <> child,
                  content_dir <> parent_path <> "/" <> renamed,
                )
              Nil
            }
            True -> {
              Nil
            }
          }
        })
      // comment ascendors siblings and uncomment the anscedors themselves
      comment_ascendors_siblings(parent_path)
    }
  }
}

fn spotlight(path) {
  //Un comment our file/dir and it's children
  comment(path, False)

  let splits = string.split(path, "/")
  let splits_poped = list.take(splits, list.length(splits) - 1)
  let parent_path = string.join(splits_poped, "/")
  let name = result.unwrap(list.last(splits), "")
  let real_name = case is_commented(parent_path, name) {
    True -> {
      "#" <> name
    }
    False -> {
      name
    }
  }
  let real_path = parent_path <> "/" <> real_name
  un_comment_children(parent_path, real_name)

  comment_ascendors_siblings(real_path)
}

pub fn main() {
  case argv.load().arguments {
    [command, dir_path] -> {
      case command {
        "comment" -> comment(dir_path, True)
        "uncomment" -> comment(dir_path, False)
        "spotlight" -> spotlight(dir_path)
        _ -> io.println("Usage: spotlighter comment/spotlight <path>")
      }
    }
    [command] -> {
      case command {
        "comment" -> comment_children("", "")
        "uncomment" -> un_comment_children("", "")
        "spotlight" -> io.println("spotlight something inside content dir")
        _ -> io.println("Usage: spotlighter comment/spotlight <path>")
      }
    }
    _ -> io.println("Usage: spotlighter comment/spotlight <path>")
  }
}
