# rover

## Purpose 

This is a simple command line utility that allows for the quick retrieval of datasets.
The datasets are defined in a `roverfile` using TOML in the user's config directory. 
This allows for the custom definitions of commonly required datasets.

I don't believe that this will be particularly useful to anyone other than myself.
However, this serves two purposes

1. Implements this desired functionality that I need
2. Let's me work with rust

## TODO 

- [ ] Add documentation for roverfile syntax
- [ ] Ability to dispatch to different sources/filenames/etc. based on current architecture
- [ ] Add ability to specify local sources for files
- [ ] Better error handling
- [ ] More available data fields for file metadata and dataset values
  - Arbitrary data?
- [ ] Asynchronous downloads of files
