# Publishing

## Prequisites

1. Set up your `crates.io` API token: https://doc.rust-lang.org/cargo/reference/publishing.html.

## GitHub Release

1. Merge your branch with package version `x.y.z` to the `main` branch on GitHub via a pull request.
1. Pull the `main` branch in your local repository.

    ```
    git pull origin main
    ```

1. Create a new tag and add a message.

    ```
    git tag -a x.y.z -m "Release version x.y.z"
    ```

1. Push the tag to the remote repository.

    ```
    git push origin x.y.z
    ```

1. Release on GitHub.

## Publish to `crates.io`

1. Publish to `crates.io`.

    ```
    cargo publish
    ```