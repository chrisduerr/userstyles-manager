## Userstyles Manager

This userstyles manager automatically downloads userstyles and prints them to `stdout`. It was made to work with the `userContent.css`, so all you need to do is put the output of the userstyles manager into it.

To add styles you need to add their `id` to the `userstyles.toml`, you can see the `id` in the `userstyles.org` url. Just add a name for it using `[{name}]` and then add the id below it using `id = {id}`. Now when you run it, it will download the default settings and print the styles to `stdout`. If you want to change settings of a style you can change them after downloading it once. A description of type or available settings is in the comment after each field.

To update a style or get the results after changing your settings, just run it again. You can see an example of a `userstyles.toml` with id and settings ![here](https://github.com/chrisduerr/userstyles-manager/blob/master/userstyles.toml).
