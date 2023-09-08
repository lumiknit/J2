pub static SH_INIT: &str = r#"
# luminkit's jump helper 2
# To initialize this for your shell, run:
# eval "$(j2 init)"
# To initialize this for your shell permanently, add the above line to your shell's rc file.
# Set default values
if [ -z "$HOME" ]; then
  export HOME=~
fi
if [ -z "$J2_IGNORES" ]; then
  export J2_IGNORES="node_modules:target:dist:venv:env:build:out:output:bin:obj:lib:libs:include:includes:vendor:assets:resources:res:tmp:test:tests"
fi
if [ -z "$J2_JONE_PATH" ]; then
  export J2_JONE_PATH="$HOME/.J2-jones"
fi
if [ -z "$J2_EDITOR" ]; then
  export J2_EDITOR="vi"
fi
# Create functions
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
      echo "  J2_JONE_PATH: The path to store jone files (default: ~/.J2-jones)"
      echo "  J2_EDITOR: The command name of editor to edit jone notes (default: vi)"
      echo "Shortcuts:"
      echo "  j <QUERY>: Find a directory and cd"
      echo "  j-+ [<NAME>]: Create a new jone & section with name."
      echo "  j-- [<NAME>]: Create a new jone & section and move to the section in the jone NAME."
      echo "  j- [<NAME>]: Move to the latest section in the jone NAME."
      echo "  j_ [<NAME>]: List of sections in the jone NAME."
      echo "  j. [<NAME>]: Open the note file of the latest section in the jone "
      ;;
  esac
}
__J2_LIST() {
  $__J2 jone-list
}
j() {
  if [ $# -eq 0 ]; then
    if command -v fzf 2>&1 >/dev/null && command -v fd 2>&1 >/dev/null; then
      p=$({
        while read -r d; do
          fd -c never -t d . "$d" 2>/dev/null
        done <<< ${J2_FIND_BASE_PATHS//:/$'\n'}
      } | fzf)
      if [ $? -eq 0 ]; then
        cd "$p"
      fi
    else
      echo "J2 Error: No query is given" >&2
      echo "  If fzf and fd is installed, you can find directory interactively" >&2
    fi
  else
    J cd $@
  fi
}
j-+() {
  J jone-new $@
}
j-() {
  p="$($__J2 jone-latest $@)"
  cd "$p"
}
j--() {
  j-+ $@
  j- $@
}
j_() {
  J jone-sections $@
}
j.() {
  J jone-note $@
}
complete -W "version find cd pushd clone jone-new jone-list jone-sections jone-note" J
complete -F __J2_LIST "j--"
complete -F __J2_LIST "j-+"
complete -F __J2_LIST "j-"
complete -F __J2_LIST "j_"
complete -F __J2_LIST "j."
"#;
