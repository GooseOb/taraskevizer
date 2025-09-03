#!/bin/bash

# Define colors for directories
colors=(
	"lightblue"
	"lightgreen"
	"lightyellow"
	"lightpink"
	"lightcyan"
	"orange"
	"gold"
	"violet"
	"salmon"
	"khaki"
)

# Ignore patterns (regex OR glob). Examples:
ignore_patterns=(
	"node_modules"
	"dist"
	"build"
	".*\.test\.ts$"
	".*\.spec\.ts$"
)

# Function: check if a file matches an ignore pattern
should_ignore() {
	local f="$1"
	for pat in "${ignore_patterns[@]}"; do
		if [[ "$f" =~ $pat ]]; then
			return 0
		fi
	done
	return 1
}

# Recursively find .ts files, filter ignored
mapfile -t all_files < <(find . -type f -name "*.ts")
files=()
for f in "${all_files[@]}"; do
	if ! should_ignore "$f"; then
		files+=("$f")
	fi
done

declare -A imports
declare -A dir_color
declare -A file_dir

# Normalize paths
normalize_path() {
	realpath --relative-to="." "$1" 2>/dev/null || echo "$1"
}

# Assign a color to each directory
color_index=0
for file in "${files[@]}"; do
	dir=$(dirname "$file")
	if [[ -z "${dir_color[$dir]}" ]]; then
		dir_color["$dir"]="${colors[$((color_index % ${#colors[@]}))]}"
		((color_index++))
	fi
	file_dir["$(normalize_path "$file")"]="$dir"
done

# Collect imports and exports
for file in "${files[@]}"; do
	[[ -f "$file" ]] || continue
	matches=()

	while IFS= read -r line; do
		if [[ $line =~ (import|export).*\'(.+)\'\; ]]; then
			imported="${BASH_REMATCH[2]}"
			dir=$(dirname "$file")
			full_import="$dir/$imported"

			if [[ -f "$full_import.ts" ]]; then
				matches+=("$(normalize_path "$full_import.ts")")
			elif [[ -f "$full_import/index.ts" ]]; then
				matches+=("$(normalize_path "$full_import/index.ts")")
			elif [[ -f "$full_import" ]]; then
				matches+=("$(normalize_path "$full_import")")
			fi
		fi
	done <"$file"

	if [[ ${#matches[@]} -gt 0 ]]; then
		imports["$(normalize_path "$file")"]="${matches[*]}"
	else
		imports["$(normalize_path "$file")"]="NONE"
	fi
done

# Generate DOT graph
echo "digraph G {"
echo "  rankdir=LR;"
echo "  node [style=filled];"

# Nodes
for file in "${!imports[@]}"; do
	dir="${file_dir[$file]}"
	color="${dir_color[$dir]}"
	echo "  \"$file\" [fillcolor=$color];"
done

# Edges
for file in "${!imports[@]}"; do
	if [[ ${imports[$file]} != "NONE" ]]; then
		for imported in ${imports[$file]}; do
			# Skip ignored imports
			if should_ignore "$imported"; then
				continue
			fi
			echo "  \"$file\" -> \"$imported\";"
		done
	fi
done
echo "}"
