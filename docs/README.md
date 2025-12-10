# Documentation

This directory contains the documentation site built with [mdBook](https://rust-lang.github.io/mdBook/).

## Building Locally

Install mdBook:
```bash
cargo install mdbook
```

Build and serve the documentation:
```bash
mdbook serve
```

Then open http://localhost:3000 in your browser.

## Building for Production

```bash
mdbook build
```

The output will be in the `book/` directory.

## Automatic Deployment

Documentation is automatically built and deployed to GitHub Pages on every push to the main branch via GitHub Actions.

See `.github/workflows/deploy-docs.yml` for the deployment configuration.
