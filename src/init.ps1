# luminkit's jump helper 2
# Put this script in your PowerShell profile script.

if (-not $env:HOME) {
	$env:HOME = $HOME
}
if (-not $env:J2_IGNORE) {
	$env:J2_IGNORE = "$env:HOME\.J2_ignore"
}
if (-not $env:J2_JONE_PATH) {
	$env:J2_JONE_PATH = "$env:HOME\.J2_jones"
}
if (-not $env:J2_EDITOR) {
	$env:J2_EDITOR = "notepad"
}

# Create functions
$global:__J2 = "<EXECUTABLE_PATH>"

function __J2_find {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	$dirs = & $global:__J2 find @args
	if ($LASTEXITCODE -ne 0) {
		return $null
	} elseif ($dirs.Count -ne 1) {
		# Maybe help
		$dirs | ForEach-Object { Write-Output $_ }
		return $null
	} else {
		return $dirs
	}
}

function J {
	param (
		[string]$command,
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	switch ($command) {
		"version" {
			& $global:__J2 --version
		}
		"find" {
			__J2_find @args
		}
		"cd" {
			$dirs = __J2_find @args
			if ($dirs) {
				Write-Output "J2: cd to $dirs"
				Set-Location $dirs
			}
		}
		"pushd" {
			$dirs = __J2_find @args
			if ($dirs) {
				Write-Output "J2: pushd to $dirs"
				Push-Location $dirs
			}
		}
		"edit" {
			$dirs = __J2_find @args
			if ($dirs) {
				Write-Output "J2: edit $dirs"
				& $env:J2_EDITOR $dirs
			}
		}
		"clone" {
			& $global:__J2 clone @args
		}
		"jone-new" {
			& $global:__J2 jone-new @args
		}
		"jone-list" {
			& $global:__J2 jone-list
		}
		"jone-sections" {
			& $global:__J2 jone-sections @args
		}
		"jone-note" {
			$p = & $global:__J2 jone-latest @args
			& $env:J2_EDITOR "$p\README.md"
		}
		"help" {
			@"
<INIT_HELP>
"@ | Write-Output
		}
		default {
			$dirs = __J2_find $command @args
			if ($dirs) {
				Write-Output "J2: cd to $dirs"
				Set-Location $dirs
			}
		}
	}
}

function j! {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	J "edit" @args
}

function j-+ {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	J "jone-new" @args
}

function j- {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	$p = & $global:__J2 jone-latest @args
	Set-Location $p
}

function j-! {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	$p = & $global:__J2 jone-latest @args
	& $env:J2_EDITOR $p
}

function j-- {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	j-+ @args
	j- @args
}

function j--! {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	j-+ @args
	j-! @args
}

function j_ {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	J "jone-sections" @args
}

function j. {
	param (
		[Parameter(ValueFromRemainingArguments=$true)]
		[string[]]$args
	)
	J "jone-note" @args
}

# To initialize this for your shell, you should add the script to your PowerShell profile script.
# For example, execute the following command:
# Add-Content $PROFILE $(j2 shell-init pwsh)
# Then, restart your shell or execute the following command:
# . $PROFILE
