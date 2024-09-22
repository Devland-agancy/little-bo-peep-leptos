import argv
import gleam/io
import gleam/iterator
import gleam/list
import gleam/result
import gleam/string
import simplifile

const content_dir = "../src/content/"

// fn rename(result, splits) {
//   case splits {
//     [] -> {
//       ""
//     }
//     [only_one] -> {
//       string.append(result, "#" <> only_one)
//     }
//     [name, ..rest] -> {
//       rename(string.append(result, name <> "/"), rest)
//     }
//   }
// }

fn is_commented(parent_path, name) {
  let dir_children =
    result.unwrap(simplifile.read_directory(content_dir <> parent_path), [])

  let is_commented = list.any(dir_children, fn(child) { child == "#" <> name })

  is_commented
}

fn rename(from, to) {
  let _ = case result.unwrap(simplifile.is_file(from), True) {
    True -> {
      simplifile.rename_file(from, to)
    }
    False -> {
      simplifile.rename_directory(from, to)
    }
  }
}

fn comment(path, comment_out) {
  // comment_out means we add # at start of file if not already added
  // let dir_children = simplifile.read_directory(path)
  // let splits = string.split(path, "/")
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
  // case simplifile.is_directory(content_dir <> path) {
  //   Ok(is_dir) -> {
  //     let _ = case is_dir {
  //       True -> {
  //         simplifile.rename_directory(content_dir <> path, renamed)
  //       }
  //       False -> {
  //         simplifile.rename_directory(content_dir <> path, renamed)
  //       }
  //     }
  //     io.println("hidden")
  //   }
  //   Error(err) -> {
  //     io.println("Could not open file")
  //   }
  // }
  // case dir_children {
  //   Ok(dir_children) -> {
  //     case dir_children {
  //       [] -> {
  //         Ok("No files found")
  //       }
  //       [child, ..] -> {
  //         Ok(child)
  //       }
  //     }
  //   }
  //   Error(err) -> {
  //     io.println("Could not open dir")
  //     Error(err)
  //   }
  // }
}

pub fn main() {
  case argv.load().arguments {
    [comment_or_spotlight, dir_path] -> {
      //let path = "../src/content/" <> dir_path
      case comment_or_spotlight {
        "comment" -> comment(dir_path, True)
        "uncomment" -> comment(dir_path, False)
        "spotlight" -> comment(dir_path, False)
        _ -> io.println("Usage: spotlighter comment/spotlight <path>")
      }
    }
    _ -> io.println("Usage: spotlighter comment/spotlight <path>")
  }
}
