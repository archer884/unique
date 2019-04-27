# Unique

> A `unique(1)` pipeline filter

In all likelihood, your computer already has a command that does this, and the odds are you can *probably* get by with either of them. Read on to discover if `unique` can be useful to you.

## Windows

PowerShell (`pwsh`) offers the `Select-Object` cmdlet which, called with the `-Unique` flag, does almost exactly what `unique` does. I imagine that `Select-Object` is using the CLR's `GetHashCode()` mechanism, but I am far too lazy to dig into this to find out. This command operates on object streams rather than text streams, which makes it more versatile than `unique`. For working with text streams, however, you can expect `unique` to offer slightly better performance characteristics due to the innate inefficiency of string handling in `pwsh` and in CLR languages generally.

## macOS

Of course, `pwsh` is also available on macOS, but it isn't installed by default. Considerations are identical.

If instead you use a shell like `bash`, your built-in option is `uniq(1)`, which is more efficient than both `Select-Object` and `unique`, but which will also fail to filter non-consecutive repetitions. As a demonstration, try the following command using the test file in this repository: `cat ./resource/test-file.txt | uniq`. The string "One" will appear twice in the output. This is by design: `uniq` is written for maximum efficiency and, as such, does not retain the full text being filtered in memory, which is necessary to discover non-consecutive repetitions.

## The `unique` advantage

`unique` differs slightly from both of these commands.

### Efficient operation on text streams

In comparison to the CLR-based `Select-Object`, `unique` offers more efficient operation on text streams.

This is because `unique` reads the entirety of stdin at once before printing only unique lines. Only a single input buffer is ever allocated, with the resulting individual lines being represented in memory as slices of that buffer rather than as discrete strings, as would be necessary in the CLR.

> Note: Yes, I know that the CLR now has `Span<T>`. Whether or not this technology has been incorporated into `Select-Object` is not known to me. Feel free to look it up and let me know. :)

### Successful detection of non-consecutive repetitions

Allowing for the necessary reduction in sheer efficiency, `unique` offers an advantage over the built-in command `uniq` in that it will faithfully exclude repeated lines even when those repetitions are not back to back. The primary difference you'll see as a user is that your pipelines can be shorter (`uniq` must often be combined with `sort` to address this shortcoming) and you can expect your input and output strings to appear in the same order (because, obviously, you didn't have to sort them). Compare the following pipelines:

```shell
cat foo.txt | sort | uniq
```

```shell
cat foo.txt | unique
```

*You will save five entire characters.* Your children will thank you.

### Inverted mode

`unique` also offers a brand new `--invert` mode, which causes the filter to print *only repeated items* instead. What are you gonna do with that? I dunno, go wild.
