
if [ "$1" = "patch" ]; then
	(cd version && CARGO_RUN_VERSION_TYPE="patch" cargo run)
elif [ "$1" = "major" ]; then
	(cd version && CARGO_RUN_VERSION_TYPE="major" cargo run)
elif [ "$1" = "minor" ]; then
	(cd version && CARGO_RUN_VERSION_TYPE="minor" cargo run)
else
  echo "Argumento no válido"
fi
