# NAME

lecho - Line Echo

# SYNOPSIS

**lecho** **-f** *file* **-l** *line* \[**-c**\] \[**-d** *delimiter*\]
\[**-h**\]

# DESCRIPTION

**lecho** reads a file and then echos only the specified line of it. If
the -c is used **lecho** will treat the file as a Comma-Separated-Values
file and split it at the delimiter and echo the contents at the index 1.

# OPTIONS

**-f *file***

:   the file to read

**-l *line***

:   the line to print

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

zockerfreunde03/z0gg3r \<zockerfreunde03.info\@gmx.de>
