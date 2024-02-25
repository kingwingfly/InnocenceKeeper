# InnocenceKeeper
Periodically check user's status, if no response, safely encrypt sensitive files on the disk.

# Develop
```sh
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON .  # Generate compile_commands.json
mkdir target
cmake .
make
```
