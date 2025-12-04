#!/bin/bash

if [ -z "$1" ]; then
	DAY=$(date +%d)
else
	DAY=$(printf "%02d" "$1")
fi

if [ -z "$2" ]; then
	YEAR=$(date +%Y)
else
	YEAR="$2"
fi

if [[ ${#1} -eq 4 ]]; then
	YEAR="$1"
	DAY=$(date +%d)
fi

mkdir -p "$YEAR" && cd "$YEAR" || exit

cargo new --lib "day_$DAY"
if [ $? -ne 0 ]; then
	echo "Error: Failed to create the Cargo project with 'cargo new'."
	exit 1
fi

cd "day_$DAY" || exit

cargo add lib --path ../../lib
if [ $? -ne 0 ]; then
	echo "Error: Failed to add the lib dependency with 'cargo add'."
	exit 1
fi

cat >"src/lib.rs" <<'EOL'
pub mod part_1;
pub mod part_2;
EOL

PART_TEMPLATE=$(
	cat <<'EOF'
pub fn day_x(input: &str) -> usize {
  todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#""#;

        let result = day_x(input);
        assert_eq!(0, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = day_x(input);
        assert_eq!(0, result);
    }
}
EOF
)

echo "$PART_TEMPLATE" >src/part_1.rs
echo "$PART_TEMPLATE" >src/part_2.rs

touch "src/input"

echo "Project day_$DAY created successfully!"
