# Semantic Versioning

**Semantic Versioning (SemVer)** is a versioning scheme that provides a standardized way to assign version numbers to software projects, making it easy to track changes, understand compatibility, and manage dependencies. In SemVer, version numbers follow the format:

```
MAJOR.MINOR.PATCH
```

Each segment has a specific meaning:

1. **MAJOR**: Introduces incompatible changes that break backward compatibility.
1. **MINOR**: Adds new features in a backward-compatible manner.
1. **PATCH**: Fixes bugs or makes minor changes that are backward-compatible.

Example of a Semantic Version:

```
2.3.5
```

- `2`: MAJOR version.
- `3`: MINOR version.
- `5`: PATCH version.

### Rules of Semantic Versioning:

1. **Increment the MAJOR** version when you make incompatible API changes.
   - Example: If you change a function signature in a way that breaks existing code, you would increment the major version.
   - Before: 1.5.0 → After: 2.0.0
1. **Increment the MINOR** version when you add functionality in a backward-compatible manner.
   - Example: Adding a new method that doesn’t affect existing code but adds extra features.
   - Before: 1.5.0 → After: 1.6.0
1. **Increment the PATCH** version when you make backward-compatible bug fixes.
   - Example: Fixing a bug or a typo that doesn't affect the API.
   - Before: 1.5.0 → After: 1.5.1

### Why Use Semantic Versioning?

Semantic versioning helps communicate changes clearly, ensures compatibility with dependencies, and allows developers to handle updates easily without unexpected issues. It's commonly used in package management systems like NPM (Node.js), PyPI (Python), and others to help developers control which versions of libraries or modules they use.

### Example in a Real-World Project

Imagine you're developing a library and need to version it using SemVer. Here's how you might version it over time:

1. **Initial Release (`1.0.0`)**:
   - You create the first version of your library with basic functionality.
   - Version: `1.0.0`
1. **Adding Features (`1.1.0`)**:
   - You add a new feature that enhances your library but doesn't break existing functionality.
   - Version: `1.1.0`
1. **Bug Fixes (1.1.1)**:
   - You find and fix a minor bug that doesn't affect the API or break compatibility.
   - Version: `1.1.1`
1. **Incompatible Changes (`2.0.0`)**:
   - After feedback, you decide to refactor your library, changing the way users interact with some parts, breaking compatibility with the previous version.
   - Version: `2.0.0`

### Example in NPM (Node.js)

In a Node.js project using NPM, dependencies are often listed in the `package.json` file, with their versions specified using SemVer notation. Here's an example of how versions can be specified:

```json
{
  "name": "my-node-app",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21",
    "express": "~4.17.1"
  }
}
```

- `^4.17.21`: Allows updates that do not change the major version. For example, `4.18.0` or `4.19.0` would be acceptable, but `5.0.0` would not.
- `~4.17.1`: Allows updates that only change the patch version, meaning `4.17.2` would be acceptable, but `4.18.0` would not.

### Version Ranges in SemVer

1. **Caret (`^`)**: Allows non-breaking updates, meaning it accepts any updates within the same major version.
   - Example: `^1.2.3` allows `1.3.0` and `1.9.0`, but not `2.0.0`.
1. **Tilde (`~`)**: Allows updates within the same minor version, so it will accept patch-level updates.
   - Example: `~1.2.3` allows `1.2.4` and `1.2.9`, but not `1.3.0`.
1. **Exact version**: You can specify an exact version to use.
   - Example: `1.2.3` only installs exactly version `1.2.3`.
1. **Wildcard (`*`)**: Allows any version.
   - Example: `*` will allow any version of the package.

### Example of Dependency Versioning

```json
{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.17.1", // Accepts 4.x.x updates
    "mongoose": "~5.10.9", // Accepts 5.10.x updates
    "lodash": "4.17.21" // Fixed version
  }
}
```

- **express**: Will accept updates to any `4.x.x` version (e.g., `4.18.0`).
- **mongoose**: Will accept updates to any `5.10.x` version (e.g., `5.10.10`).
- **lodash**: Will only accept the exact version `4.17.21`.

### Summary of Semantic Versioning Rules

1. **MAJOR version**: Incremented when you make incompatible API changes.
1. **MINOR version**: Incremented when you add backward-compatible features.
1. **PATCH version**: Incremented when you make backward-compatible bug fixes.
