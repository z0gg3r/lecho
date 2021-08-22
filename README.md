# NAME

lecho - Line Echo

# SYNOPSIS

**lecho** **-l** *line* \[**-f** *file*\] \[**-c**\] \[**-d**
*delimiter*\] \[**-h**\]

# DESCRIPTION

**lecho** reads input either from stdin or a file and outputs only the
specified line.

# OPTIONS

**-l *line***

:   the line to print

**-f *file***

:   the file to print

**-c**

:   treat the file as a .csv

**-d *delimiter***

:   set a custom delimiter

**-h**

:   display the help message

# NOTES

**lecho** expects files to be valid CSV if given the -c option. If your
file uses delimiters that deviate from the standard (that is a comma),
please use the -d option.

# SEE ALSO

RFC 4180 (CSV Standard)

# AUTHOR

zockerfreunde03/z0gg3r
