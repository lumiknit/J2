pub static SH_INIT: &str = r#"
# luminkit's jump helper 2
# To initialize this for your shell, run:
# eval "$(j2 init)"
# To initialize this for your shell permanently, add the above line to your shell's rc file.
__J2="<EXECUTABLE_PATH>"
__J2_find_first() {
  # Find directory;
  dirs=($($__J2 find $@))
  if [ $? -ne 0 ]; then
    return 1
  elif [ ${#dirs[@]} -eq 0 ]; then
    echo "J2 Error: Cannot find the path '$@'!" >&2
    return 1
  elif [ ${#dirs[@]} -eq 1 ]; then
    echo "${dirs[1]}"
  else
    # Print first 8 matches
    echo "J2: Multiple matches:" >&2
    for ((i=1; i<=8; i++)); do
      echo "  ${dirs[i]}" >&2
    done
    if [ ${#dirs[@]} -gt 8 ]; then
      echo "  ..." >&2
    fi
    echo "${dirs[1]}"
  fi
}
J() {
  case "$1" in
    version)
      $__J2 version
      ;;
    find|f)
      $__J2 find ${@:2}
      ;;
    cd|c)
      # Change directory
      dirs=$(__J2_find_first ${@:2})
      if [ $? -eq 0 ]; then
        echo "J2: cd to $dirs"
        cd "$dirs"
      fi
      ;;
    pushd|push|pus|pu|p)
      # Push directory
      dirs=$(__J2_find_first ${@:2})
      if [ $? -eq 0 ]; then
        echo "J2: pushd to $dirs"
        pushd "$dirs"
      fi
      ;;
    clone|C)
      # Clone git repository
      $__J2 clone ${@:2}
      ;;
    jone-new|new|N)
      # Create a new jone
      $__J2 jone-new ${@:2}
      ;;
    jone-list|list|l)
      # List jones;
      $__J2 jone-list
      ;;
    jone-sections|sections|s)
      # List sections in the jone
      $__J2 jone-sections ${@:2}
      ;;
    jone-note|note|n)
      # Edit jone notes
      p="$($__J2 jone-latest ${@:2})"
      $J2_EDITOR "$p/README.md"
      ;;
    *)
      # Print help message
      echo "luminkit's jump helper 2"
      echo "Usage: J <COMMAND> [ARGS]"
      echo "Commands:"
      echo "  version: Print the version"
      echo "  find <QUERY>: Find a directory"
      echo "  cd <QUERY>: Change directory"
      echo "  pushd <QUERY>: Push directory"
      echo "  clone <REPO_URL>: Clone a git repository"
      echo "  jone-new [<NAME>]: Create a new jone (j-zone)"
      echo "  jone-list: List jones"
      echo "  jone-sections [<NAME>]: List sections in the jone"
      echo "  jone-note [<NAME>]: Edit jone notes"
      echo "Environment variables:"
      echo "  J2_REPOS_DIR: The directory where git repositories are stored"
      echo "  J2_FIND_BASE_PATHS: The base paths to find directories (separated by ':')"
      echo "  J2_IGNORES: The directories to ignore when finding (separated by ':')"
      echo "  J2_JONE_PATH: The path to store jone files"
      echo "  J2_EDITOR: The command name of editor to edit jone notes (e.g. vi)"
      echo "Shortcuts:"
      echo "  j <QUERY>: Find a directory and cd"
      echo "  j-- [<NAME>]: Create a new jone with name."
      echo "  j- [<NAME>]: Move to the latest section in the jone NAME."
      echo "  j. [<NAME>]: Open the note file of the latest section in the jone "
      ;;
  esac
}
__J2_LIST() {
  $__J2 jone-list
}
j() {
  J cd $@
}
j--() {
  J jone-new $@
}
j-() {
  p="$($__J2 jone-latest $@)"
  cd "$p"
}
j.() {
  J jone-note $@
}
complete -W "version find cd pushd clone jone-new jone-list jone-sections jone-note" J
complete -F __J2_LIST "j--"
complete -F __J2_LIST "j-"
complete -F __J2_LIST "j."
"#;