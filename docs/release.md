# Release

## Steps

1. Make sure CI is green
2. Update version in `Cargo.toml`
3. Update version in `lib/y/version.rb`
4. Create a tag matching the version `git tag -a vx.x.x -m "Release version vx.x.x"` 
5. Push tag to GitHub repo: `git push vx.x.x`
6. Create a GitHub release (requires GitHub CLI): `gh release create vx.x.x ./build/out/*/*.gz`
7. Package and upload gem: `gem build && gem push y-rb-x.x.x.gem`

## Cross-compile binaries (to be released as assets on GitHub)

There is a script in `./build` that automates a fair chunk of work, but it is
not bulletproof. I mainly use it to compile for arm64 platforms on my machine,
and then upload manually. Most binaries (especially x86) should come from CI.

It also includes a workaround for Alpine (popular in Docker images) what usually
requires to be a static lib.

Run the following script. It produces a bunch of archives in `./build/out/`.
You need to upload the produced assets manually after the GitHub release was
created.

```bash
./build/build.rb
```

## Future work

With this [PR](https://github.com/rubygems/rubygems/pull/5175) merged into
rubygems, we will most likely be able to rely on the extension to manage builds
for us in the future.

Pre-build binaries are not verified, and are inherently a security concern.
