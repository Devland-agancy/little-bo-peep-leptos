import argv
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import shellout
import simplifile

fn path_from_reversed_path(reversed_path: List(String)) -> String {
  reversed_path
  |> list.reverse
  |> string.join("/")
}

fn drop_last(s: String, char: String) {
  case string.ends_with(s, char) {
    True -> string.drop_right(s, 1)
    False -> s
  }
}

fn reversed_path_from_path(path: String) -> List(String) {
  path
  |> drop_last("/")
  |> string.split("/")
  |> list.reverse
}

fn protected_from_commenting(name: String) -> Bool {
  string.starts_with(name, "__parent") || string.starts_with(name, ".")
}

fn add_hashtag_if_appropriate(name: String) -> String {
  case string.starts_with(name, "#") || protected_from_commenting(name) {
    True -> name
    False -> "#" <> name
  }
}

fn remove_hashtag_if_there(name: String) -> String {
  case string.starts_with(name, "#") {
    True -> string.drop_left(name, 1)
    False -> name
  }
}

fn add_or_remove_hashtag_if_appropriate(
  name: String,
  want_a_hashtag: Bool,
) -> String {
  case want_a_hashtag {
    True -> add_hashtag_if_appropriate(name)
    False -> remove_hashtag_if_there(name)
  }
}

fn comment_or_uncomment_if_possible(
  name: String,
  reversed_parent_path: List(String),
  want_commented: Bool,
) -> Nil {
  let new_name = add_or_remove_hashtag_if_appropriate(name, want_commented)
  let old_name = add_or_remove_hashtag_if_appropriate(name, !want_commented)
  let new_reversed_path_pieces = list.prepend(reversed_parent_path, new_name)
  let old_reversed_path_pieces = list.prepend(reversed_parent_path, old_name)
  let new_path = path_from_reversed_path(new_reversed_path_pieces)
  let old_path = path_from_reversed_path(old_reversed_path_pieces)
  rename_if_different(old_path, new_path)
}

fn comment_if_possible(name: String, reversed_parent_path: List(String)) {
  comment_or_uncomment_if_possible(name, reversed_parent_path, True)
}

fn uncomment_if_possible(name: String, reversed_parent_path: List(String)) {
  comment_or_uncomment_if_possible(name, reversed_parent_path, False)
}

fn rename_if_different(from: String, to: String) -> Nil {
  case from == to {
    True -> Nil
    False -> {
      io.println("trying to rename " <> from <> " to " <> to)
      let _ =
        shellout.command(run: "git", in: ".", with: ["mv", from, to], opt: [])
      Nil
    }
  }
}

fn spotlight_recursive(reversed_path_pieces: List(String)) -> Nil {
  case reversed_path_pieces {
    [] -> Nil
    [first, ..rest] -> {
      let sibling_names =
        result.unwrap(
          simplifile.read_directory(path_from_reversed_path(rest)),
          [],
        )
      case list.any(sibling_names, string.starts_with(_, "__parent")) {
        False -> Nil
        True -> {
          list.each(sibling_names, comment_if_possible(_, rest))
          uncomment_if_possible(first, rest)
          spotlight_recursive(rest)
        }
      }
    }
  }
}

fn comment_recursive(
  reversed_path_pieces: List(String),
  want_commented: Bool,
  including_root: Bool,
) -> Nil {
  case reversed_path_pieces {
    [] -> Nil
    [first, ..rest] -> {
      let new_first = case including_root {
        True -> add_or_remove_hashtag_if_appropriate(first, want_commented)
        False -> first
      }
      let new_reversed_path_pieces = list.prepend(rest, new_first)
      let new_path = path_from_reversed_path(new_reversed_path_pieces)
      let old_path = path_from_reversed_path(reversed_path_pieces)
      rename_if_different(old_path, new_path)
      result.unwrap(simplifile.read_directory(new_path), [])
      |> list.each(fn(name) {
        comment_recursive(
          list.prepend(new_reversed_path_pieces, name),
          want_commented,
          True,
        )
      })
    }
  }
}

// *****************
// ORIGINAL API FUNCTIONS
// *****************

fn spotlight(path) {
  let pieces = reversed_path_from_path(path)
  comment_recursive(pieces, False, False)
  spotlight_recursive(pieces)
}

fn comment(path, want_commented: Bool) {
  case reversed_path_from_path(path) {
    [] -> Nil
    [first, ..rest] ->
      comment_or_uncomment_if_possible(first, rest, want_commented)
  }
}

pub fn main() {
  let args = argv.load().arguments
  case args {
    [command, dir_path] -> {
      case command {
        "comment" -> comment(dir_path, True)
        "uncomment" -> comment(dir_path, False)
        "spotlight" -> spotlight(dir_path)

        _ -> io.println("Usage: spotlighter comment/spotlight <path>")
      }
    }
    _ -> io.println("Usage: spotlighter comment/spotlight <path>")
  }
}
