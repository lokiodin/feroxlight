# Feroxlight

Tool to highlight with customazible colors (red, blue, green, cyan, yellow and magenta) a text given in stdin or by a specified file


## USAGE
```
USAGE:
    feroxlight.exe [OPTIONS]

OPTIONS:
        --cb <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in blue
        --cc <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in cyan
        --cg <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in green
        --cm <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in magenta
        --cr <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in red
        --cy <REGEX1,REGEX2>    Highlight specified regex (delimited by a comma) in yellow
    -f, --file <FILE>           Will search in the specified file
    -h, --help                  Print help information
        --no-default            Do not use the default regex and color
```

You can specify multiple regex for each color by delimiting them with a comma `,`. 

## Examples
TODO

## TODO
- [ ] add a config file to avoid specifying regex for each color (maybe see https://docs.rs/crate/argfile/latest)
- [ ] add more default regex
- [ ] Add more info in the help (regex with flags (https://docs.rs/regex/latest/regex/#syntax), examples, ...)