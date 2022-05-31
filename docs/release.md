# Release

## Steps

1. Make sure CI is green
2. Update version in `Cargo.toml`
3. Update version in `lib/y/version.rb`
4. Create a tag matching the version `git tag -a v0.1.1 -m "Release version v0.1.1"` 
5. Push tag to GitLab repo
