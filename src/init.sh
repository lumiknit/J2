# luminkit's jump helper 2
# Use with: eval "$(j2 shell-init)"

if [ -z "$HOME" ]; then
  export HOME=~
fi
if [ -z "$J2_IGNORE" ]; then
  export J2_IGNORE="$HOME/.J2_ignore"
fi
if [ -z "$J2_JONE_PATH" ]; then
  export J2_JONE_PATH="$HOME/.J2_jones"
fi
if [ -z "$J2_EDITOR" ]; then
  export J2_EDITOR="vi"
fi
# Create functions
__J2="<EXECUTABLE_PATH>"
__J2_find() {
	IFS=$'\n'
	dirs=($($__J2 find $@))
	if [ $? -ne 0 ]; then
		return 1
	elif [ ${#dirs[@]} -ne 1 ]; then
		# Maybe help
		for d in "${dirs[@]}"; do
			echo $d
		done
		return 1
	else
		echo ${dirs[1]}
	fi
}
J() {
  case "$1" in
    version)
      $__J2 --version
      ;;
    find|f)
      __J2_find ${@:2}
      ;;
    cd|c)
      # Change directory
      dirs=$(__J2_find ${@:2})
      if [ $? -eq 0 ]; then
        echo "J2: cd to $dirs"
        cd "$dirs"
      fi
      ;;
    pushd|push|pus|pu|p)
      # Push directory
      dirs=$(__J2_find ${@:2})
      if [ $? -eq 0 ]; then
        echo "J2: pushd to $dirs"
        pushd "$dirs"
      fi
      ;;
    edit|edi|ed|e)
      # Edit with default editor
      dirs=$(__J2_find ${@:2})
      if [ $? -eq 0 ]; then
        echo "J2: edit $dirs"
        $J2_EDITOR "$dirs"
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
			cat << EOF
<INIT_HELP>
EOF
      ;;
  esac
}
__J2_LIST() {
  $__J2 jone-list
}
j() {
  J cd $@
}
j!() {
  J edit $@
}
j-+() {
  J jone-new $@
}
j-() {
  p="$($__J2 jone-latest $@)"
  cd "$p"
}
j-!() {
  p="$($__J2 jone-latest $@)"
  $J2_EDITOR "$p"
}
j--() {
  j-+ $@
  j- $@
}
j--!() {
  j-+ $@
  j-! $@
}
j_() {
  J jone-sections $@
}
j.() {
  J jone-note $@
}
complete -W "version find cd pushd edit clone jone-new jone-list jone-sections jone-note" J
complete -F __J2_LIST "j--!"
complete -F __J2_LIST "j--"
complete -F __J2_LIST "j-+"
complete -F __J2_LIST "j-!"
complete -F __J2_LIST "j-"
complete -F __J2_LIST "j_"
complete -F __J2_LIST "j."
complete -F __J2_LIST "j!"

# To initialize this for your shell, run:
# eval "$(j2 shell-init)"
# To initialize this for your shell permanently, add the above line to your shell's rc file.
# Set default values
