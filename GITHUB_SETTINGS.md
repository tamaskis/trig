# GitHub Settings

The following settings were modified from the default settings in the **Settings** tab.

- **General**
    - **Pull Requests**
        - Disabled **Allow merge commits**.
        - Disabled **Allow rebase merging**.
        - Enabled **Automatically delete head branches**.
- **Branches**
    - **Branch protection rules**
        1. Clicked on **Add branch protection rule**.
        1. Set **Branch name pattern** to `main`.
        1. Enabled **Require a pull request before merging.**
            1. Enabled **Dismiss stale pull request approvals when new commits are pushed**
            1. Enabled **Require review from Code Owners**.
            1. Enabled **Require approval of the most recent reviewable push**.
        1. Enabled **Require status checks to pass before merging**.
            1. Enabled **Require branches to be up to date before merging**.
        1. Enabled **Require conversation resolution before merging**.
        1. Enabled **Require linear history**.
        1. Enabled **Lock branch**.