import argv
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/result
import gleam/string
import shellout
import simplifile

const ins = string.inspect

fn path_from_reversed_path(reversed_path: List(String)) -> String {
  reversed_path
  |> list.reverse
  |> string.join("/")
}

fn drop_last(s: String, char: String) {
  case string.ends_with(s, char) {
    True -> string.drop_end(s, 1)
    False -> s
  }
}

fn inverted_path_pieces(path: String) -> List(String) {
  path
  |> drop_last("/")
  |> string.split("/")
  |> list.reverse
}

fn protected_from_commenting(name: String) -> Bool {
  string.starts_with(name, "__parent") || string.starts_with(name, ".") || string.contains(name, "/.")
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

fn exists(path: String) -> Bool {
  result.unwrap(simplifile.is_file(path), False) || result.unwrap(simplifile.is_directory(path), False)
}

fn rename_if_different(from: String, to: String) -> Nil {
  case to != from && exists(from) {
    False -> Nil
    True -> {
      io.println("renaming " <> from <> " to " <> to)
      let _ =
        shellout.command(run: "git", in: ".", with: ["mv", from, to], opt: [])
      Nil
    }
  }
}

fn read_directory(inverted_path: List(String)) -> List(String) {
  result.unwrap(simplifile.read_directory(
    path_from_reversed_path(inverted_path)),
    []
  )
}

fn uncomment_ancestors(
  inverted_path: List(String)
) -> Nil {
  case inverted_path {
    [] -> Nil
    [first, ..rest] -> {
      let sibling_names = read_directory(rest)
      case !list.any(sibling_names, string.ends_with(_, ".emu")) {
        True -> Nil
        False -> {
          uncomment_if_possible(first, rest)
          uncomment_ancestors(rest)
        }
      }
    }
  }
}

fn spotlight_recursive(inverted_path: List(String)) -> Nil {
  case inverted_path {
    [] -> Nil
    [first, ..rest] -> {
      let sibling_names = read_directory(rest)
      case !list.any(sibling_names, string.ends_with(_, ".emu")) {
        True -> Nil
        False -> {
          list.each(sibling_names, comment_if_possible(_, rest))
          uncomment_if_possible(first, rest)
          spotlight_recursive(rest)
        }
      }
    }
  }
}

fn comment_or_uncomment_descendants(
  inverted_path: List(String),
  comment: Bool,
  including_self: Bool,
) -> Nil {
  case inverted_path {
    [] -> Nil
    [first, ..rest] -> {
      let first = case including_self {
        True -> add_or_remove_hashtag_if_appropriate(first, comment)
        False -> first
      }
      let anti_first = case including_self {
        True -> add_or_remove_hashtag_if_appropriate(first, !comment)
        False -> first
      }
      let new_reversed_path_pieces = [first, ..rest]
      let old_reversed_path_pieces = [anti_first, ..rest]
      let new_path = path_from_reversed_path(new_reversed_path_pieces)
      let old_path = path_from_reversed_path(old_reversed_path_pieces)
      rename_if_different(old_path, new_path)
      // io.println("after trying to rename " <> old_path <> " to " <> new_path <> " (" <> ins(comment) <> ")")
      let files = result.unwrap(simplifile.read_directory(new_path), [])
      // io.println("found files " <> ins(files) <> " in directory " <> new_path)
      files
      |> list.each(fn(name) {
        comment_or_uncomment_descendants(
          list.prepend(new_reversed_path_pieces, name),
          comment,
          True,
        )
      })
    }
  }
}

fn get_root(
  inverted_path: List(String)
) -> List(String) {
  case inverted_path {
    [] -> []
    [first, ..rest] -> {
      case first == ".." {
        True -> inverted_path
        False -> {
          let sibling_names = read_directory(rest)
          case !list.any(sibling_names, string.ends_with(_, ".emu")) {
            True -> inverted_path
            False -> get_root(rest)
          }
        }
      }
    }
  }
}

fn first_satisfying(l: List(a), condition: fn(a) -> Bool) -> Option(a) {
  case l {
    [] -> None
    [first, ..rest] -> case condition(first) {
      True -> Some(first)
      False -> first_satisfying(rest, condition)
    }
  }
}

fn all_equal_evaluations(l: List(a), f: fn(a) -> b) -> Result(Nil, #(a, a)) {
  case l {
    [] -> Ok(Nil)
    [first, ..rest] -> {
      let w = f(first)
      case first_satisfying(rest, fn(z) { f(z) != w }) {
        Some(z) -> Error(#(first, z))
        None -> all_equal_evaluations(rest, f)
      }
    }
  }
}

fn all_equal(l: List(a)) -> Bool {
  case l {
    [] -> True
    [first, ..rest] -> case list.all(rest, fn(z) { z == first }) {
      False -> False
      True -> all_equal(rest)
    }
  }
}

fn on_true_on_false(b: Bool, f1: fn() -> a, f2: fn() -> a) -> a {
  case b {
    True -> f1()
    False -> f2()
  }
}

fn on_error_on_ok(res: Result(a, b), f1: fn(b) -> c, f2: fn(a) -> c) -> c {
  case res {
    Error(thing) -> f1(thing)
    Ok(thing) -> f2(thing)
  }
}

fn announce_error(message: String) -> fn() -> Nil {
  fn() { io.println(message) }
}

pub fn main() {
  let paths = argv.load().arguments

  use <- on_true_on_false(
    list.is_empty(paths),
    announce_error("spotlight error: no paths given"),
  )

  let assert [first, ..] = paths
  let assert Ok(last) = list.last(paths)

  let paths = case string.starts_with(last, "-show") {
    True -> paths |> list.reverse |> list.drop(1) |> list.reverse
    False -> paths
  }

  let f = fn(path) {
    inverted_path_pieces(path) |> get_root |> list.reverse |> string.join("/")
  }

  use _ <- on_error_on_ok(
    all_equal_evaluations(paths, f),
    fn (pair) {
      let #(first, second) = pair
      io.println("spotlight error: non-equal root directories for paths:")
      io.println("- " <> first <> " -> " <> f(first))
      io.println("- " <> second <> " -> " <> f(second))
    }
  )

  let root_pieces = first |> inverted_path_pieces |> get_root

  case string.starts_with(last, "-show") {
    False -> Nil
    True -> {
      let root_string = root_pieces |> list.reverse |> string.join("/")
      case simplifile.get_files(root_string) {
        Error(e) -> io.println("spotlight error: cannot get files for root dir " <> root_string <> ": " <> ins(e))
        Ok(files) -> {
          let n = string.length(root_string)
          let message_finish = case last == "-show-only" {
            True -> ":"
            False -> " before spotlighting:"
          }
          io.println("\nfiles in " <> root_string <> message_finish)
          list.each(files, fn (filename) { io.println("- " <> {filename |> string.drop_start(n + 1)})})
        }
      }
    }
  }

  use <- on_true_on_false(
    last == "-show-only",
    fn() { Nil },
  )

  io.println("")

  comment_or_uncomment_descendants(root_pieces, True, False)

  list.each(
    paths,
    fn (path) {
      let pieces = path |> inverted_path_pieces
      comment_or_uncomment_descendants(
        pieces,
        False,
        True,
      )
      uncomment_ancestors(pieces)
    }
  )

  case string.starts_with(last, "-show") {
    False -> Nil
    True -> {
      let root_string = root_pieces |> list.reverse |> string.join("/")
      case simplifile.get_files(root_string) {
        Error(e) -> io.println("spotlight error: cannot get files for root dir " <> root_string <> " after spotlighting: " <> ins(e))
        Ok(files) -> {
          let n = string.length(root_string)
          io.println("\nfiles in " <> root_string <> " after spotlighting:")
          list.each(files, fn (filename) { io.println("- " <> {filename |> string.drop_start(n + 1)})})
        }
      }
    }
  }

  Nil
}
