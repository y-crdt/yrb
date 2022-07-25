# Release

## Steps

1. Make sure CI is green
2. Update version in `Cargo.toml`
3. Update version in `lib/y/version.rb`
4. Create a tag matching the version `git tag -a vx.x.x -m "Release version vx.x.x"` 
5. Push tag to GitHub repo: `git push vx.x.x`
6. Create a GitHub release (requires GitHub CLI): `gh release create vx.x.x ./build/out/*/*.gz`
7. Package and upload gem: `gem build && gem push y-rb-x.x.x.gem`
